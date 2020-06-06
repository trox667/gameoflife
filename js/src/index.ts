const WIDTH = 800
const HEIGHT = 600

window.addEventListener('load', () => {
  const canvas: HTMLCanvasElement = document.getElementById(
    'canvas'
  ) as HTMLCanvasElement
  const context = canvas.getContext('2d')

  const imgData = context.createImageData(WIDTH, HEIGHT)
  const index = (x: number, y: number) => (y * WIDTH + x) * 4
  for (let y = 0; y < HEIGHT; y++) {
    for (let x = 0; x < WIDTH; x++) {
      imgData.data[index(x,y)] = 255
      imgData.data[index(x,y)+1] = 0
      imgData.data[index(x,y)+2] = 0
      imgData.data[index(x,y)+3] = 255
    }
  }

  const render = () => {
    if (context) {
      context.putImageData(imgData, 0, 0)
    }
    window.requestAnimationFrame(render)
  }
  window.requestAnimationFrame(render)
})

