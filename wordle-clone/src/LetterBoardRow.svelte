<script lang="ts">
    import ColorHint from './ColorHint.svelte';

    export let rowString: string;
    export let showHints: boolean;
    export let correctWord: string;

    $: correct = (letter, index) => {
        return correctWord[index] === letter;
    };

    $: misplaced = (letter, index) => {
        return correctWord.includes(letter) && !correct(letter, index);
    };
</script>

        <!-- To take up the grid space -->
        <div />
    {#each rowString as letter, idx}
        <div class:correct={showHints && correct(letter, idx)} class:misplaced={showHints && misplaced(letter, idx)}
             class="letterBox">{letter}</div>
    {/each}
    {#if showHints}
        <ColorHint color={"#" + rowString} />
    {:else}
        <!-- To take up the grid space -->
        <div />
    {/if}

<style>
    .letterBox {
        background-color: lightgray;
        display: inline-block;
        width: 2.8ch;
        height: 2.8ch;
        padding: 0.25em;
        margin: 0.25em;
        text-align: center;
        border-radius: 10%;
        padding-top: 0.5em;
    }

    .correct {
        background-color: green;
    }

    .misplaced {
        background-color: yellow;
    }
</style>
