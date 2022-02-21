<script lang="ts">
    import Keyboard from './Keyboard.svelte';
    import LetterBoard from './LetterBoard.svelte';

    const color = "FF552E";
    let correctLetters = [];
    let wrongLetters = [];
    let misplacedLetters = [];
    let pastGuesses = [];
    let currentGuess = "";

    const processLetter = (event) => {
        if(currentGuess.length < color.length) {
            let letter = event.detail;
            currentGuess = [...currentGuess, letter];
        }
    };

    $: processGuess = (event) => {
        if(currentGuess.length === color.length) {
            for (let [index, letter] of currentGuess.entries()) {
                if(letter === color[index]) {
                    correctLetters = [...correctLetters, letter];
                } else if (color.includes(letter)) {
                    misplacedLetters = [...misplacedLetters, letter]
                } else {
                    wrongLetters = [...wrongLetters, letter];
                }
            }
            pastGuesses = [...pastGuesses, currentGuess];
            currentGuess = "";
        }
    };

    const backspace = (event) => {
       currentGuess = currentGuess.slice(0, -1);
    };

    const cssVariable = `--color:#${color}`;

</script>

<div style="text-align: center">
<div class="colorBlock" style={cssVariable} />
</div>

<LetterBoard correctWord={color} guesses={pastGuesses} {currentGuess} />
<Keyboard {correctLetters} {wrongLetters} {misplacedLetters} on:key={processLetter} 
    on:submit={processGuess} on:backspace={backspace}/>

<style>
    .colorBlock {
        height: 10em;
        width: 10em;
        background-color: var(--color);
        display: inline-block;
        border: 2px ridge darkgrey; 
        margin-bottom: 30px;
    }
</style>
