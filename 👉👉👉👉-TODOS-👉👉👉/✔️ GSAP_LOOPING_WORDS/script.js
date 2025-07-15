
// GSAP Looping Words Animation
document.addEventListener("DOMContentLoaded", () => {
  // DOM Elements
  const wordList = document.querySelector("[data-looping-words-list]");
  const selector = document.querySelector("[data-looping-words-selector]");
  const words = Array.from(wordList.children);
  
  // Configuration
  const TOTAL_WORDS = words.length;
  const WORD_HEIGHT_PERCENT = 100 / TOTAL_WORDS;
  const ANIMATION_DURATION = 1.2;
  const SELECTOR_UPDATE_DURATION = 0.5;
  const PAUSE_BETWEEN_WORDS = 2;
  const RESET_THRESHOLD = 3;
  
  // State
  let currentIndex = 0;
  
  // Update selector width to match current word
  function updateSelectorWidth() {
    const nextIndex = (currentIndex + 1) % TOTAL_WORDS;
    const nextWord = words[nextIndex];
    const wordWidth = nextWord.getBoundingClientRect().width;
    const listWidth = wordList.getBoundingClientRect().width;
    const widthPercent = (wordWidth / listWidth) * 100;
    
    gsap.to(selector, {
      width: `${widthPercent}%`,
      duration: SELECTOR_UPDATE_DURATION,
      ease: "expo.out"
    });
  }
  
  // Move to next word
  function moveToNextWord() {
    currentIndex++;
    
    // Animate word list movement
    gsap.to(wordList, {
      yPercent: -WORD_HEIGHT_PERCENT * currentIndex,
      duration: ANIMATION_DURATION,
      ease: "elastic.out(1, 0.85)",
      onStart: updateSelectorWidth,
      onComplete: handleMoveComplete
    });
  }
  
  // Handle animation completion and infinite loop
  function handleMoveComplete() {
    if (currentIndex >= TOTAL_WORDS - RESET_THRESHOLD) {
      // Move first word to end for infinite loop
      wordList.appendChild(wordList.children[0]);
      currentIndex--;
      
      // Reset position without animation
      gsap.set(wordList, { 
        yPercent: -WORD_HEIGHT_PERCENT * currentIndex 
      });
      
      // Update words array to match DOM
      words.push(words.shift());
    }
  }
  
  // Initialize
  updateSelectorWidth();
  
  // Start animation loop
  gsap.timeline({ repeat: -1, delay: 1 })
    .call(moveToNextWord)
    .to({}, { duration: PAUSE_BETWEEN_WORDS })
    .repeat(-1);
});