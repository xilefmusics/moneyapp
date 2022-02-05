<script>
  import ViewDashboard from './ViewDashboard.svelte';
  import ViewOverviewHistory from './ViewOverviewHistory.svelte';
  import ViewPodHistory from './ViewPodHistory.svelte';
  import ViewBudgetHistory from './ViewBudgetHistory.svelte';
  import ViewPod from './ViewPod.svelte';
  import ViewBooking from './ViewBooking.svelte';
  import ViewAddBooking from './ViewAddBooking.svelte';
  import ViewBudget from './ViewBudget.svelte';
  import ViewError from './ViewError.svelte';
  import BottomSlider from './BottomSlider.svelte';
  import ViewConfig from './ViewConfig.svelte';

  import { onMount } from 'svelte';

  import api from '../api';
  import process_data from '../process_data';

  let view = localStorage.getItem('url') ? "dashboard": "config";
  const changeView = (v) => {
    view=v
    document.body.scrollTop = 0;
    document.body.scrollLeft = 0;
  };

  let pods = [];
  let budgets = [];
  let bookings = [];
  let state = {
    data: null,
    head_data: null,
  };


  onMount(async () => {
		pods = await api.getPods();
    budgets = await api.getBudgets();
    bookings = await api.getBookings();
    state.data = process_data(pods, budgets, bookings);
    const date = new Date();
    state.head_data = state.data[date.getFullYear()].months[date.getMonth() + 1];
	});

</script>

<style>
  #main {
    position: fixed;
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    flex-wrap: nowrap;
    justify-content: flex-start;
    align-items: stretch;
  }
  #view-wrapper {
    flex-grow: 1;
    overflow-x: auto;
    overflow-y: auto;
  }
</style>

<main>
  <div id='main'>
    <div id='view-wrapper'>
      {#if view === 'dashboard'}
        <ViewDashboard
          changeView={changeView}
          data={state.head_data}
        />
      {:else if view === 'overviewHistory'}
        <ViewOverviewHistory
          changeView={changeView}
          data={state.data}
        />
      {:else if view === 'podHistory'}
        <ViewPodHistory
          changeView={changeView}
          pods={pods}
          data={state.data}
        />
      {:else if view === 'budgetHistory'}
        <ViewBudgetHistory
          changeView={changeView}
          budgets={budgets}
          data={state.data}
        />
      {:else if view === 'pod'}
        <ViewPod />
      {:else if view === 'budget'}
        <ViewBudget />
      {:else if view === 'booking'}
        <ViewBooking />
      {:else if view === 'addBooking'}
        <ViewAddBooking />
      {:else if view === 'config'}
        <ViewConfig />
      {:else}
        <ViewError />
      {/if}
    </div>
    <BottomSlider 
      changeView={changeView}
      view={view}
    />
  </div>
</main>
