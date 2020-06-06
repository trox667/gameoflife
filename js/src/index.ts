const WIDTH = 800
const HEIGHT = 600

enum CellStatus {
  DEAD,
  ALIVE,
}

interface Cell {
  status: CellStatus
}

type Board = Array<Array<Cell>>

const createBoard = (width: number, height: number, random = true): Board => {
  let board = new Array(height)
  for (let y = 0; y < height; y++) {
    board[y] = new Array(width)
  }
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      if (random) {
        board[y][x] =
          Math.random() > 0.5
            ? { status: CellStatus.ALIVE }
            : { status: CellStatus.DEAD }
      } else {
        board[y][x] = { status: CellStatus.DEAD }
      }
    }
  }
  return board
}

const renderBoard = (board: Board, imgData: ImageData) => {
  for (let y = 0; y < HEIGHT; y++) {
    for (let x = 0; x < WIDTH; x++) {
      const idx = index(x, y)
      const cell = board[y][x]
      if (cell.status === CellStatus.ALIVE) {
        imgData.data[idx] = 0
        imgData.data[idx + 1] = 255
        imgData.data[idx + 2] = 0
      } else {
        imgData.data[idx] = 255
        imgData.data[idx + 1] = 255
        imgData.data[idx + 2] = 255
      }
      imgData.data[idx + 3] = 255
    }
  }
}

const neighborCount = (board: Board, x: number, y: number): number => {
  const neighbors = new Array<Cell>()
  if (x > 0) neighbors.push(board[y][x - 1])
  if (x > 0 && y > 0) neighbors.push(board[y - 1][x - 1])

  if (x + 1 < board[y].length) neighbors.push(board[y][x + 1])
  if (x > 0 && y + 1 < board.length) neighbors.push(board[y + 1][x - 1])

  if (y > 0) neighbors.push(board[y - 1][x])
  if (x + 1 < board[y].length && y > 0) neighbors.push(board[y - 1][x + 1])

  if (y + 1 < board.length) neighbors.push(board[y + 1][x])
  if (x + 1 < board[y].length && y + 1 < board.length) neighbors.push(board[y + 1][x + 1])

  return neighbors.reduce((acc, curr) => {
    if (curr.status === CellStatus.ALIVE) return acc + 1
    return acc
  }, 0)
}

const applyRules = (board: Board, x: number, y: number): Cell => {
  const cell = board[y][x]
  const nc = neighborCount(board, x, y)
  if (cell.status === CellStatus.ALIVE) {
    if (nc < 2) return { status: CellStatus.DEAD }
    if (nc > 3) return { status: CellStatus.DEAD }
  } else {
    if (nc === 3) return { status: CellStatus.ALIVE }
  }
  return cell
}

const turn = (board: Board, width: number, height: number) => {
  const newBoard = createBoard(width, height, false)
  for (let y = 0; y < HEIGHT; y++) {
    for (let x = 0; x < WIDTH; x++) {
      newBoard[y][x] = applyRules(board, x, y)
    }
  }
  return newBoard
}

const testNeighborCount = () => {
  const board = new Array(3)
  for (let i = 0; i < 3; i++) {
    board[i] = new Array(3)
  }
  board[0][0] = { status: CellStatus.DEAD }
  board[0][1] = { status: CellStatus.DEAD }
  board[0][2] = { status: CellStatus.DEAD }

  board[1][0] = { status: CellStatus.DEAD }
  board[1][1] = { status: CellStatus.ALIVE }
  board[1][2] = { status: CellStatus.DEAD }

  board[2][0] = { status: CellStatus.DEAD }
  board[2][1] = { status: CellStatus.DEAD }
  board[2][2] = { status: CellStatus.DEAD }

  console.log(neighborCount(board, 1, 1))
}
// testNeighborCount()

const index = (x: number, y: number) => (y * WIDTH + x) * 4

window.addEventListener('load', () => {
  const canvas: HTMLCanvasElement = document.getElementById(
    'canvas'
  ) as HTMLCanvasElement
  const context = canvas.getContext('2d')

  const imgData = context.createImageData(WIDTH, HEIGHT)

  let board = createBoard(WIDTH, HEIGHT)
  let timeDiff = performance.now()
  const render = (time: DOMHighResTimeStamp) => {
    if (time - timeDiff > 1000) {
      board = turn(board, WIDTH, HEIGHT)
    }
    if (context) {
      renderBoard(board, imgData)
      context.putImageData(imgData, 0, 0)
    }
    window.requestAnimationFrame(render)
  }
  window.requestAnimationFrame(render)
})
