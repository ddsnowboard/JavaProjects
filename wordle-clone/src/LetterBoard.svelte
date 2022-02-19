<script>
    import LetterBoardRow from './LetterBoardRow.svelte';

    export let correctWord;
    export let guesses;
    export let currentGuess;
    export let rows = 6;
    export let columns = 6;

    $: rowStrings = Array.from(Array(rows).keys()).map((row) => {
        let allGuesses = [...guesses, currentGuess];
        return Array.from(Array(columns).keys()).map((col) => {
            if(row < allGuesses.length && col < allGuesses[row].length) {
                return allGuesses[row][col]
            } else {
                return " "
            }
        }).join("");
    });

    // One more column for the color hints and another to make everything look centered
    const cssVariables = `--rows:${rows};--columns:${columns + 2}`;

</script>


<div id="letterBoard" style={cssVariables}>
{#each rowStrings as row, rowIdx}
    <LetterBoardRow rowString={row} showHints={rowIdx < guesses.length} {correctWord} />
{/each}
</div>

<style>
    #letterBoard {
        display: grid;
        grid-template-rows: repeat(var(--rows), 1fr);
        grid-template-columns: repeat(var(--columns), 1fr);
        width: min(100vh, 35ch);
        justify-content: center;
        margin-left: auto;
        margin-right: auto;
    }
</style>
