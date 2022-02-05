// helpers

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

// functions

const accumulateChanges = (bookings, pods, budgets) => {
    const is_debt = pod_name => {
        const pod = pods.find(pod => pod.name === pod_name);
        return pod !== undefined && pod.type === 'debt';
    };

    const pod_changes = arrayToObject(pods.map(pod => [pod.name, 0]));
    const budget_changes = arrayToObject(budgets.map(budget => [budget.name, 0]));
    let income = 0;
    let outcome = 0;

    bookings.forEach(booking => {
        if (booking.from_pod !== 'world' && booking.from_pod !== 'fake') { pod_changes[booking.from_pod] -= booking.amount; }
        if (booking.to_pod !== 'world' && booking.to_pod !== 'fake') { pod_changes[booking.to_pod] += booking.amount; };
        if (booking.from_budget !== 'world' && booking.from_budget !== 'fake') { budget_changes[booking.from_budget] -= booking.amount; };
        if (booking.to_budget !== 'world' && booking.to_budget !== 'fake') { budget_changes[booking.to_budget] += booking.amount; };
        if (booking.from_pod === 'world' 
            && booking.from_budget === 'world'
            && booking.to_pod !== 'world'
            && booking.to_budget !== 'world') {
                if (is_debt(booking.to_pod)) {
                    income -= booking.amount;
                } else {
                    income += booking.amount;
                }
            };  
        if (booking.from_pod !== 'world' 
            && booking.from_budget !== 'world'
            && booking.to_pod === 'world'
            && booking.to_budget === 'world') {
                if (is_debt(booking.from_pod)) {
                    outcome -= booking.amount;
                } else {
                    outcome += booking.amount;
                }
        };
        if (booking.from_pod === 'amortizing' && booking.from_budget === 'amortizing') {
            outcome -= booking.amount; 
        };

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
    const acc = type => pods.filter(pod => pod.type == type).map(pod => monthPods[pod.name]).reduce((a, b) => a + b, 0);
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

const addBudgetSum = month => {
    const budget_sum = Object.entries(month.budget).map(budget => budget[1]).reduce((a, b) => a + b);
    return {...month, budget_sum};
};

// parts

const accumulateToMonths = (bookings, pods, budgets) => {
    const bookingsWithDate = bookings.map(booking => ({...booking, date: mapDate(new Date(booking.date))}));
    const twoDimBookings = Object.values(classify(bookingsWithDate, booking => booking.date.year));
    const threeDimBookings = twoDimBookings.map(bookings => Object.values(classify(bookings, booking => booking.date.month)));
    const accumumulatedBookings = threeDimBookings.map(bookings => bookings.map(bookings => ({...accumulateChanges(bookings, pods, budgets),bookings})));
    return accumumulatedBookings.flat();
};

const calcMonthValues = (months, pods) => {
    const m1 = cumulativeSumOfChange(months);
    const m2 = m1.map(month => ({
        ...month,
        pod_sums: sumPods(month.pod, pods),
        pod_change_sums: sumPods(month.pod_changes, pods),
    }));
    return m2.map(month => addBudgetSum(month));
};

const monthsToYearObject = months => {
    const y1 = Object.values(classify(months, month => month.bookings[0].date.year));
    const y2 = y1.map(year => arrayToObject(year.map(month => [month.bookings[0].date.month, month])));
    const y3 = arrayToObject(y2.map(year => [Object.values(year)[0].bookings[0].date.year, year]));
    return y3;
};

const process_data = (pods, budgets, bookings) => {
    const months = calcMonthValues(accumulateToMonths(bookings, pods, budgets), pods);
    const years = monthsToYearObject(months);
    return years;
};

const test = () => {
    const data = require('./data.json');
    const {pods, budgets, bookings} = data;
    process_data(pods, budgets, bookings);
};
// test();

export default process_data;