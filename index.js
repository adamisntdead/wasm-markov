import { Markov } from "./wasm_markov";

const textInput = document.getElementById('text-input')
const submitButton = document.getElementById('submit')
const sentenceElement = document.getElementById('sentence')

submitButton.onclick = () => {
  const input = textInput.value

  const markov = Markov.new()

  input.split('.').map(x => {
    markov.parse(x + '.')
  })

  const sentence = markov.generate_sentence()

  sentenceElement.textContent = sentence

  markov.free()
}
