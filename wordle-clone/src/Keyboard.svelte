<script lang="ts">
    import { createEventDispatcher } from 'svelte';

    export let correctLetters: string[];
    export let misplacedLetters: string[];
    export let wrongLetters: string[];

    const dispatch = createEventDispatcher();

    $: isCorrect = letter => {
        return correctLetters.includes(letter);
    };

    $: isMisplaced = letter => {
        return misplacedLetters.includes(letter) && !isCorrect(letter);
    };

    $: isWrong = letter => {
        return wrongLetters.includes(letter);
    };


    const rows = [
        "0123456789".split(""),
        "ABCDEF".split(""),
    ]
</script>

{#each rows as row, idx}
    <div class="row">
        {#if idx === rows.length - 1}
            <div class="key" on:click={() => dispatch('submit')}>
                Submit
            </div>
        {/if}
    {#each row as letter}
        <div class="key" class:misplaced={isMisplaced(letter)} class:wrong={isWrong(letter)}
             class:correct={isCorrect(letter)}
             on:click={() => dispatch('key', letter)}>
             {letter}
        </div>
    {/each}
        {#if idx === rows.length - 1}
            <div class="key" on:click={() => dispatch('backspace')}>
                Backspace
            </div>
        {/if}
    </div>
{/each}

<style>
    .key {
        display: inline-block;
        text-align: center;
        cursor: pointer;
        background-color: #bbbbbb;
        padding: 1em;
        margin: .3em;
        size: 12pt;
        border-radius: 10%;
        user-select: none;
    }

    .row {
        display: flex;
        justify-content: center;
    }

    .misplaced {
        background-color: yellow;
    }

    .correct {
        background-color: green;
    }

    .wrong {
        background-color: #555555;
    }
</style>
