<script>
    export let monthly;
    export let changeView;
    export let pods;
</script>

<style>
    #main {
        flex-grow: 1;
    }
    h1 {
        padding-left: 1em;
    }
</style>

<div id='main' class="div-fill" on:click={() => changeView('dashboard')}>
    <h1>Pod History</h1>
    {#if monthly && pods}
        <table>
            <tr>
                <th>Date</th>
                {#each pods as pod, idx}
                    <th>{pod.name}</th>
                {/each}
            </tr>
            {#each monthly as month, idx}
                <tr>
                    <td>{`${("0" + month.date.getDate()).slice(-2)}.${("0" + (month.date.getMonth() + 1)).slice(-2)}.${month.date.getFullYear()}`}</td>
                    {#each pods as pod, pod_idx}
                        <td>{(month.pods[pod.name] / 100).toFixed(2)} €</td>
                    {/each}
                </tr>
            {/each}
        </table>
    {/if}
</div>