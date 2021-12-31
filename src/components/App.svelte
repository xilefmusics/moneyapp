<script>
  import ViewDashboard from './ViewDashboard.svelte';
  import ViewOverviewHistory from './ViewOverviewHistory.svelte';
  import ViewPodHistory from './ViewPodHistory.svelte';
  import ViewBudgetHistory from './ViewBudgetHistory.svelte';
  import ViewPod from './ViewPod.svelte';
  import ViewBooking from './ViewBooking.svelte';
  import ViewBudget from './ViewBudget.svelte';
  import ViewError from './ViewError.svelte';
  import BottomSlider from './BottomSlider.svelte';
  import ViewConfig from './ViewConfig.svelte';

  import { onMount } from 'svelte';

  import api from '../api';
  import monthly from '../math/monthly';

  let view = localStorage.getItem('url') ? "dashboard": "config";
  const changeView = (v) => {view=v};

  let pods = [];
  let budgets = [];
  let bookings = [];
  let state = {
    head: null,
    monthly: null,
  };


  onMount(async () => {
		pods = await api.getPods();
    budgets = await api.getBudgets();
    bookings = await api.getBookings();
    state.monthly = monthly(pods, budgets, bookings, new Date(2021, 9, 1));
    state.head = state.monthly[state.monthly.length -1];
	});

</script>

<style>
  #main {
    display: flex;
    flex-direction: column;
    flex-wrap: nowrap;
    justify-content: flex-start;
    align-items: stretch;
  }
</style>

<main>
  <div id='main' class='div-fill'>
      {#if view === 'dashboard'}
        <ViewDashboard
          state={state.head}
          changeView={changeView}
        />
      {:else if view === 'overviewHistory'}
        <ViewOverviewHistory
          monthly={state.monthly}
          changeView={changeView}
        />
      {:else if view === 'podHistory'}
        <ViewPodHistory
          monthly={state.monthly}
          changeView={changeView}
          pods={pods}
        />
      {:else if view === 'budgetHistory'}
        <ViewBudgetHistory
          monthly={state.monthly}
          changeView={changeView}
          budgets={budgets}
        />
      {:else if view === 'pod'}
        <ViewPod />
      {:else if view === 'budget'}
        <ViewBudget />
      {:else if view === 'booking'}
        <ViewBooking />
      {:else if view === 'config'}
        <ViewConfig />
      {:else}
        <ViewError />
      {/if}
      <!--<BottomSlider
        changeView={changeView}
        />-->
  </div>
</main>
