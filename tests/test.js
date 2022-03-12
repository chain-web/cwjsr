function  test() {
  let num = Math.random()
  if (num > 0.5) {
    console.log('up')
  } else {
    console.log('down')
  }
  return num
}
//boa engine 默认会把 js code string 中最后执行函数的返回值作为返回值，给到外层调用方
test()