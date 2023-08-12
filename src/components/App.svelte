<script>
    import wasm from "../wasm/Cargo.toml";

    let bindings = {
        add: () => {},
        init_wasm: () => {},
        pythagorean: () => {}
    };

    async function initWasm() {
        bindings = await wasm();

        console.log({bindings});

        bindings.init_wasm();
    }

    initWasm();

    let a = 1;
    let b = 2;

    $: sum = bindings.add(a, b);
    $: pythagorean = bindings.pythagorean(a, b);
</script>

<input type="number" bind:value={a}>
<input type="number" bind:value={b}>
<p>{sum}</p>
<p>{pythagorean}</p>