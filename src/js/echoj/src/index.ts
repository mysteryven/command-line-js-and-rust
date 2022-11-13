import cac from 'cac'
const cli = cac('echo')

cli.option('--n', 'Do not print new line')
cli.version("0.1.0")
cli.command("<...TEXT>", "Input Text").action((text, options) => {
  if (!options.n) {
    text.push("\n")
  }
  process.stdout.write(text.join(" "))
})

const parsed = cli.parse()