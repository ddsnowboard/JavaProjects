<script>
    export let correctWord;
    export let guesses;
    export let rows = 6;
    export let columns = 5;

    $: rowStrings = Array.from(Array(rows).keys()).map((row) => {
        return Array.from(Array(columns).keys()).map((col) => {
            if(row < guesses.length && col < guesses[row].length) {
                return guesses[row][col]
            } else {
                return " "
            }
        }).join("");
    });
</script>


{#each rowStrings as row}
    <div class="row">
    {#each row as letter, idx}
        <div class="letterBox">{letter}</div>
    {/each}
    </div>
{/each}

<style>
    .row div {
        background-color: lightgray;
        display: inline-block;
        width: 2.8ch;
        height: 2.8ch;
        padding: 0.25em;
        margin: 0.25em;
        border-radius: 10%;
    }

    .row {
        text-align: center;
        display: flex;
        justify-content: center;
    }
</style>
