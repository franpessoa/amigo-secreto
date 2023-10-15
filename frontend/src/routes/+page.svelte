<script>
    import { onDestroy, onMount } from "svelte";

    let participants = []
    let name = ""
    let email = ""
    let seed = false;
    let seed_content = "";
    let validity = false;

    $: validity = validateEmail(email) 

    function validateEmail(email) {
        console.log(email)
        let re = /\S+@\S+\.\S+/;
        return re.test(email);
    }


    function add() {
        console.log(validity && name !== "")

        console.log(validity)
        console.log(name !== "")
        if (validity && name !== "") {
            participants = [...participants, {name, email}]
        } else {
            alert("Dados inválidos")
        }
    }

    function removeIndex(i) {
        participants = participants.filter((_, index) => index !== i)
    }

    function download(filename, text) {
        var element = document.createElement('a');
        element.setAttribute('href', 'data:text/plain;charset=utf-8,' + encodeURIComponent(text));
        element.setAttribute('download', filename);

        element.style.display = 'none';
        document.body.appendChild(element);

        element.click();

        document.body.removeChild(element);
    }


    async function run() {
        if (seed && seed_content == "") {
            alert("Semente inválida")
            return
        } else if (participants.length <= 2) {
            alert("Você precisa de pelo menos três participantes")
            return
        } else if (confirm("Sortear?")) {
            let body = {
                participants
            }
            
            if (seed) {
                body["seed"] = seed_content
            }

            let res = await fetch("/game", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(body)
            })

            if (res.status != 200) {
                res = res.json()
                alert(res["error"])
                download("erro.json", JSON.stringify(res))
            } else {
                res = res.json()
                alert(`Sorteio concluído com semente ${res["seed"]}`)
                download("resultado.json", JSON.stringify(res))
            }
        }
    }
</script>

<br><br>
<main>
    <h1>Criar um jogo de amigo secreto</h1>
    <div class="grid">
        <article id="form-container">
            <h2>Adicionar participante</h2>
            <div class="grid">
                <div class="email-container">
                    <label for="email-input">Email</label>
                    <input type="email" name="email" id="email-input" class:invalid={!validity} bind:value={email}>
                </div>
                <div id="email-container">
                    <label for="name-input">Nome</label>
                    <input type="text" name="name" id="name-input" bind:value={name}>
                </div>
            </div>
            <button on:click={add}>Adicionar</button>
        </article>
        <article style="height: fit-content;">
            <h2>Sorteio</h2>
            <label for="seed">Usar semente?  <input id="seed" type="checkbox" bind:checked={seed}></label>

            {#if seed}
                <input type="text" style="padding-top: 20px;" placeholder="Semente" bind:value={seed_content}>
            {/if}
            <button on:click={run} style="margin-top: 20px;">Sortear</button>
        </article>

    </div>

    <table>
        <thead>
            <tr>
                <th scope="col">Nome</th>
                <th scope="col">Email</th>
                <th scope="col">Remover</th>
            </tr>
        </thead>
        <tbody>
        {#each participants as p, i}
            <tr>
                <th scope="row">{p.name}</th>
                <td>{p.email}</td>
                <td><button type="button" on:click={() => removeIndex(i)}>Remover</button></td>
            </tr>
        {/each}
        </tbody>
    </table>
</main>

<style>
    main {
        max-width: 75%;
        margin: auto;
    }

    button {
        max-width: fit-content;
    }

    input[type="checkbox"]
    {
        vertical-align:middle;
    }

    article {
        height: fit-content;
    }
</style>