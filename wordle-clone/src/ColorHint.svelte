<script lang="ts">
    export let color: string;

    $: cssVariable = `--color:${color}`;
</script>

<div style={cssVariable} />

<style>
    div {
        background-color: var(--color);
        margin-left: 0.25em;
        margin-bottom: 0.25em;
    }
</style>
