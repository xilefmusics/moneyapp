const mapDate = date => ({
    year: date.getFullYear(), 
    month: date.getMonth() + 1
});

const classify = (array, func) => {
    let result = {};
    array.forEach(element => {
        const identifier = func(element);
        if (result[identifier] == undefined) {
            result[identifier] = [];
        }
        result[identifier].push(element);
    });
    return result;
};

const arrayToObject = (array) => {
    return array.reduce((object, elem) => {
        const [key, value] = elem;
        object[key] = value;
        return object;
    }, {});
};

const addObjects = (o1, o2) => arrayToObject(Object.keys(o1).map(key => [key, o1[key] + o2[key]]));

const accumulateChanges = (bookings, pods, budgets) => {
    const pod_changes = arrayToObject(pods.map(pod => [pod.name, 0]));
    const budget_changes = arrayToObject(budgets.map(budget => [budget.name, 0]));
    let income = 0;
    let outcome = 0;

    bookings.forEach(booking => {
        if (booking.from_pod !== 'world' && booking.from_pod !== 'fake') { pod_changes[booking.from_pod] -= booking.amount; }
        if (booking.to_pod !== 'world' && booking.to_pod !== 'fake') { pod_changes[booking.to_pod] += booking.amount; };
        if (booking.from_budget !== 'world' && booking.from_budget !== 'fake') { pod_changes[booking.from_budget] -= booking.amount; };
        if (booking.to_budget !== 'world' && booking.to_budget !== 'fake') { pod_changes[booking.to_budget] += booking.amount; };
        if (booking.from_pod === 'world' 
                && booking.from_budget === 'world'
                && booking.to_pod !== 'world'
                && booking.to_budget !== 'world') {income += booking.amount;};
            if (booking.from_pod !== 'world' 
                && booking.from_budget !== 'world'
                && booking.to_pod === 'world'
                && booking.to_budget === 'world') {outcome += booking.amount;};
    });

    return {pod_changes, budget_changes, income, outcome};
};

const cumulativeSumOfChange = (bookings) => {
    let result = [{
        ...bookings[0],
        pod: bookings[0].pod_changes,
        budget: bookings[0].budget_changes,
    }];

    bookings.slice(1).forEach( booking => {
        const pod = addObjects(result[result.length-1].pod, booking.pod_changes);
        const budget = addObjects(result[result.length-1].budget, booking.budget_changes);
        result.push({
            ...booking,
            pod,
            budget,
        });
    });

    return result;
};

const sumPods = (monthPods, pods) => {
    const acc = type => pods.filter(pod => pod.type == type).map(pod => monthPods[pod.name]).reduce((a, b) => a + b);
    const solvent = acc('solvent');
    const insolvent = acc('insolvent');
    const debt = acc('debt');
    const sum = solvent + insolvent;
    const real = sum - debt;
    return {
        'solvent': solvent,
        'insolvent': insolvent,
        'debt': debt,
        'sum': sum,
        'real': real,
    };
};

const process_data = (pods, budgets, bookings) => {
    const bookingsMapped = bookings.map(booking => ({...booking, date: mapDate(new Date(booking.date))}));
    const monthsWithChanges = Object.values(classify(bookingsMapped, booking => booking.date.year))
        .map(bookings => Object.values(classify(bookings, booking => booking.date.month)))
        .map(bookings => bookings.map(bookings => ({
            ...accumulateChanges(bookings, pods, budgets),
            bookings,
        }))).flat();
    const monthsWithCurrent = cumulativeSumOfChange(monthsWithChanges);
    const monthWithOverview = monthsWithCurrent.map(month => ({
        ...month,
        pod_sums: sumPods(month.pod, pods),
        pod_change_sums: sumPods(month.pod_changes, pods),
    }));

    const years = arrayToObject(Object.values(classify(monthWithOverview, month => month.bookings[0].date.year))
        .map(year => classify(year, month => month.bookings[0].date.month))
        .map(year => [Object.values(year)[0][0].bookings[0].date.year, year]));

        return years;
};

const test = () => {
    const data = require('./data.json');
    const {pods, budgets, bookings} = data;
    console.log(process_data(pods, budgets, bookings)[2020][6]);
};

//test();
export default process_data;