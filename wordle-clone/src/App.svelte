<script lang="ts">
    import Keyboard from './Keyboard.svelte';
    import LetterBoard from './LetterBoard.svelte';

    let correctLetters = ["C"];
    let wrongLetters = ["W"];
    let misplacedLetters = ["M"];

    const processLetter = (event) => {
        let letter = event.detail;
        correctLetters = [...correctLetters, letter];
    };

</script>

<LetterBoard correctWord="rocks" guesses={[correctLetters,wrongLetters,misplacedLetters]} />
<Keyboard {correctLetters} {wrongLetters} {misplacedLetters} on:key={processLetter}/>
