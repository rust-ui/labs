const WORDS = [
  'Productivity',
  'Efficiency',
  'Passion',
  'Creativity',
]

let count = 0
const SWAPPER = document.querySelector('span:nth-of-type(2)')
const SWAP = () => {
  if (!document.startViewTransition) {
    SWAPPER.innerText = WORDS[(count += 1) % WORDS.length]
  } else {
    document.startViewTransition(() => {
      SWAPPER.innerText = WORDS[(count += 1) % WORDS.length]  
    })
  }
}

setInterval(SWAP, 2000)

