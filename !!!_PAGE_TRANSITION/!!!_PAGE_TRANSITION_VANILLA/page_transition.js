// ============================================================================
// Utility Functions (matching src/shared/utils.ts)
// ============================================================================

function createTransform(from, to) {
  const scaleX = to.width / from.width;
  const scaleY = to.height / from.height;

  return new DOMMatrix()
    .translate(to.left - scaleX * from.left, to.top - scaleY * from.top)
    .scale(scaleX, scaleY)
    .toString();
}

// Page transition wrapper (mimics usePageTransition hook)
function createPageTransition({ beforeChange, afterChange, done } = {}) {
  let startResolver;
  let transitionData;
  let viewTransition;
  const reducedMotionMedia = matchMedia('(prefers-reduced-motion: reduce)');

  // Store callback to trigger afterChange (mimics useLayoutEffect)
  window.__triggerAfterChange = () => {
    if (afterChange && transitionData && viewTransition) {
      afterChange(transitionData, viewTransition);
    }
    if (startResolver) {
      startResolver();
      startResolver = null;
    }
  };

  return async function startTransition({ classNames = [], data } = {}) {
    if (!('startViewTransition' in document) || reducedMotionMedia.matches) {
      return;
    }

    return new Promise((resolve) => {
      transitionData = data;
      document.documentElement.classList.add(...classNames);

      const transition = document.startViewTransition(async () => {
        // Resolve immediately - allows caller to update DOM
        resolve();
        // Wait for afterChange to be called (mimics useLayoutEffect timing)
        await new Promise((res) => (startResolver = res));
      });

      viewTransition = transition;

      // Call beforeChange BEFORE DOM update
      beforeChange?.(data, transition);

      globalThis.ongoingTransition = transition;

      transition.finished
        .finally(() => {
          globalThis.ongoingTransition = undefined;
          document.documentElement.classList.remove(...classNames);
          done?.(data);
        })
        .catch(() => {});
    });
  };
}

// ============================================================================
// Router Logic (matching src/client/App/router.ts)
// ============================================================================

const PageType = {
  Thumbs: 0,
  Video: 1,
  Unknown: 2,
};

const pageTypeClassNames = {
  [PageType.Thumbs]: 'thumbs',
  [PageType.Video]: 'video',
  [PageType.Unknown]: 'unknown',
};

function getPageType(url) {
  if (url === '/' || url.includes('index.html') || url.endsWith('!!!_PAGE_TRANSITION_VANILLA/')) {
    return PageType.Thumbs;
  }
  if (url.includes('video.html')) {
    return PageType.Video;
  }
  return PageType.Unknown;
}

// Store state like the React component does
let thumbnailRect;
let fullEmbedRect;
const elementsToUntag = [];

// Create the transition handler with lifecycle callbacks
const startTransition = createPageTransition({
  beforeChange({ to, from, fromType, toType, clickedLink }, viewTransition) {
    if (fromType === PageType.Thumbs && toType === PageType.Video) {
      // Use the clicked link passed from the navigation handler
      if (clickedLink) {
        const thumb = clickedLink.querySelector('.video-thumb');
        thumbnailRect = thumb.getBoundingClientRect();
        elementsToUntag.push(clickedLink);
        clickedLink.style.viewTransitionName = 'embed-container';
      }
    } else if (fromType === PageType.Video && toType === PageType.Thumbs) {
      fullEmbedRect = document
        .querySelector('.embed-container')
        ?.getBoundingClientRect();
    }
  },

  afterChange({ from, fromType, toType, videoId }, viewTransition) {
    if (fromType === PageType.Video && toType === PageType.Thumbs) {
      // Find the thumbnail by video ID
      const thumbLink =
        document.querySelector(`[data-video-id="${videoId}"]`) ||
        document.querySelector('.video-link');

      if (thumbLink) {
        const thumb = thumbLink.querySelector('.video-thumb');

        elementsToUntag.push(thumbLink);
        thumbLink.style.viewTransitionName = 'embed-container';

        viewTransition.ready
          .then(async () => {
            // For some horrible reason, scroll positions aren't updated
            // until after a microtask.
            await Promise.resolve();
            thumbnailRect = thumb.getBoundingClientRect();

            document.documentElement.animate(
              [
                {
                  transform: `translate(0px, 0px)`,
                },
                {
                  transform: createTransform(fullEmbedRect, thumbnailRect),
                },
              ],
              {
                easing: 'ease',
                duration: 300,
                fill: 'both',
                pseudoElement: '::view-transition-old(root)',
              }
            );
          })
          .catch(() => undefined);
      }
    } else if (fromType === PageType.Thumbs && toType === PageType.Video) {
      viewTransition.ready
        .then(async () => {
          // For some horrible reason, scroll positions aren't updated
          // until after a microtask.
          await Promise.resolve();

          fullEmbedRect = document
            .querySelector('.embed-container')
            ?.getBoundingClientRect();

          if (fullEmbedRect && thumbnailRect) {
            document.documentElement.animate(
              [
                {
                  transform: createTransform(fullEmbedRect, thumbnailRect),
                },
                {
                  transform: `translate(0px, 0px)`,
                },
              ],
              {
                easing: 'ease',
                duration: 300,
                fill: 'both',
                pseudoElement: '::view-transition-new(root)',
              }
            );
          }
        })
        .catch(() => undefined);
    }
  },

  done() {
    while (elementsToUntag.length) {
      const element = elementsToUntag.pop();
      element.style.viewTransitionName = '';
    }
  },
});

// ============================================================================
// Navigation API Integration
// ============================================================================


if ('navigation' in window) {
  console.log('✓ Navigation API detected');

  navigation.addEventListener('navigate', (event) => {
    if (!event.canIntercept) return;

    const currentURL = navigation.currentEntry.url;
    const currentPath = new URL(currentURL).pathname;
    const destinationURL = new URL(event.destination.url);

    if (
      destinationURL.pathname.includes('index.html') ||
      destinationURL.pathname.includes('video.html')
    ) {
      console.log(`Intercepting navigation from ${currentPath} to ${destinationURL.pathname}`);

      event.intercept({
        scroll: 'manual',
        async handler() {
          const fromType = getPageType(currentPath);
          const toType = getPageType(destinationURL.pathname);

          // Get the clicked element and video ID
          const clickedLink = event.sourceElement?.closest('.video-link');
          const currentVideoId = new URL(currentURL).searchParams.get('id');

          console.log(`Transition: ${pageTypeClassNames[fromType]} → ${pageTypeClassNames[toType]}`);
          console.log('Clicked link:', clickedLink);

          // Fetch new page content
          const html = await fetch(destinationURL.href).then((r) => r.text());
          const parser = new DOMParser();
          const doc = parser.parseFromString(html, 'text/html');

          // Start transition (resolves immediately, allowing DOM update)
          await startTransition({
            classNames: [
              `from-${pageTypeClassNames[fromType]}`,
              `to-${pageTypeClassNames[toType]}`,
            ].filter(Boolean),
            data: {
              from: currentPath,
              fromType,
              to: destinationURL.pathname,
              toType,
              clickedLink,
              videoId: currentVideoId,
            },
          });

          // Update DOM (after transition callback resolves)
          console.log('Updating DOM...');
          document.body.innerHTML = doc.body.innerHTML;
          document.title = doc.title;

          // Trigger afterChange (mimics useLayoutEffect)
          window.__triggerAfterChange?.();

          // Wait for transition to complete its work
          await (
            globalThis.ongoingTransition?.domUpdated ||
            globalThis.ongoingTransition?.updateCallbackDone
          );

          event.scroll();

          if (
            event.navigationType === 'push' ||
            event.navigationType === 'replace'
          ) {
            window.scrollTo(0, 0);
          }

          console.log('Navigation complete');
        },
      });
    }
  });
} else {
  console.error('❌ Navigation API not supported. Transitions disabled.');
  console.log('Browser support check:', {
    navigation: 'navigation' in window,
    startViewTransition: 'startViewTransition' in document,
  });
}
