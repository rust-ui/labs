// Constants
const SYNC_BUFFER_SIZE = 10
const FRAME_THRESHOLD = 10
const SCROLL_THRESHOLD = 0.5
const DRAG_THRESHOLD = 5

// Initialize drawer when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
  // Remove Pane import and replace with drawer initialization
  const drawer = document.querySelector('.drawer')
  if (!drawer) return // Exit if drawer not found
  
  const scroller = drawer.querySelector('.drawer__scroller')
  const slide = drawer.querySelector('.drawer__slide')
  const anchor = drawer.querySelector('.drawer__anchor')
  const options = {
    root: drawer,
    rootMargin: '0px 0px -1px 0px',
    threshold: 1.0,
  }

  // Data attributes and viewport are now set in shell.rs

  // Drawer mechanics
  // THIS IS ALL THE PARTS FOR THE DRAWER THAT WE COVER
  // close the drawer on snap change if === 0
  const scrollSnapChangeSupport = 'onscrollsnapchange' in window
  const scrollAnimationSupport = CSS.supports('animation-timeline: scroll()')
  if (scrollSnapChangeSupport) {
    scroller.addEventListener('scrollsnapchange', () => {
      if (scroller.scrollTop === 0) {
        drawer.dataset.snapped = true
        drawer.hidePopover()
      }
    })
  }


  let observer

  let syncer, syncs = new Array(SYNC_BUFFER_SIZE), index = 0, frame = 0
  const syncDrawer = () => {
    syncer = requestAnimationFrame(() => {
      document.documentElement.style.setProperty(
        '--closed',
        1 - scroller.scrollTop / slide.offsetHeight
      )

      if (new Set(syncs).size === 1 && syncs[0] === slide.offsetHeight) {
        frame++
      }
      if (frame >= FRAME_THRESHOLD) {
        frame = 0
        syncs = new Array(SYNC_BUFFER_SIZE)
        index = 0
        scroller.addEventListener('scroll', scrollDriver, { once: true })
      } else {
        syncs[index] = scroller.scrollTop
        index = (index + 1) % syncs.length
        syncDrawer()
      }
    })
  }

  const scrollDriver = syncDrawer

  const callback = (entries) => {
    const { isIntersecting, intersectionRatio } = entries[0]
    const isVisible = intersectionRatio === 1

    if (
      !isVisible &&
      !isIntersecting &&
      scroller.scrollTop - window.visualViewport.offsetTop <
        slide.offsetHeight * SCROLL_THRESHOLD
    ) {
      drawer.dataset.snapped = true
      drawer.hidePopover()
      observer.disconnect()
    }
  }

  const handleOut = (event) => {
    if (!drawer.contains(event.target) || !event.target) {
      window.removeEventListener('focus', handleOut, true)
      drawer.hidePopover()
    }
  }

  // reset the drawer once closed
  drawer.addEventListener('toggle', (event) => {
    if (document.documentElement.dataset.css === 'true') return
    
    if (event.newState === 'closed') {
      drawer.dataset.snapped = false
      scroller.removeEventListener('scroll', scrollDriver)
      if (syncer) cancelAnimationFrame(syncer)
      document.documentElement.style.removeProperty('--closed')
      window.removeEventListener('focus', handleOut, true)
    } else if (event.newState === 'open') {
      if (!scrollSnapChangeSupport) {
        if (!observer) observer = new IntersectionObserver(callback, options)
        observer.observe(anchor)
      }
      if (!scrollAnimationSupport) {
        scroller.addEventListener('scroll', scrollDriver, { once: true })
      }
      window.addEventListener('focus', handleOut, true)
    }
  })

  const attachDrag = (element) => {
    let startY = 0, drag = 0, scrollStart

    const reset = () => {
      startY = drag = 0
      const top = scroller.scrollTop < scrollStart * SCROLL_THRESHOLD ? 0 : scrollStart

      const handleScroll = () => {
        if (scroller.scrollTop === top) {
          document.documentElement.dataset.dragging = false
          scroller.removeEventListener('scroll', handleScroll)
        }
      }
      scroller.addEventListener('scroll', handleScroll)

      scroller.scrollTo({
        top,
        behavior: 'smooth',
      })
      handleScroll()
    }

    const handle = ({ y }) => {
      drag += Math.abs(y - startY)
      scroller.scrollTo({
        top: scrollStart - (y - startY),
        behavior: 'instant',
      })
    }
    const teardown = (event) => {
      if (event.target.tagName !== 'BUTTON') {
        reset()
      }
      document.removeEventListener('mousemove', handle)
      document.removeEventListener('mouseup', teardown)
    }
    const activate = ({ y }) => {
      startY = y
      scrollStart = scroller.scrollTop
      document.documentElement.dataset.dragging = true
      document.addEventListener('mousemove', handle)
      document.addEventListener('mouseup', teardown)
    }
    element.addEventListener('click', (event) => {
      if (drag > DRAG_THRESHOLD) event.preventDefault()
      reset()
    })
    element.addEventListener('mousedown', activate)
  }
  // Only happens on mousemove so we are only affecting the scroll position
  attachDrag(drawer)

  // Handle VisualViewport changes for iOS
  const handleResize = () => {
    document.documentElement.style.setProperty(
      '--sw-keyboard-height',
      window.visualViewport.offsetTop
    )
  }
  window.visualViewport?.addEventListener('resize', handleResize)
// THERE REALLY ISN'T THAT MUCH TO BE HONEST

}) // End DOMContentLoaded

