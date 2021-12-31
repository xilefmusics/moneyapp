<script>
    export let monthly;
    export let changeView;
    export let pods;
</script>

<style>
    #main {
        background-color: purple;
        flex-grow: 1;
    }
</style>

<div id='main' class="div-fill">
    <h1 on:click={() => changeView('dashboard')}>Pod History</h1>
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
                    <td>{`${month.date.getFullYear()}-${month.date.getMonth() + 1}-${month.date.getDate()}`}</td>
                    {#each pods as pod, pod_idx}
                        <td>{month.pods[pod.name] / 100} €</td>
                    {/each}
                </tr>
            {/each}
        </table>
    {/if}
</div>