import cumulate from './cumulate';
import cumulatePods from './cumulatePods';
import cumulateBudgets from './cumulateBudgets';
import cumulateYear from './cumulateYear';
import calcDiff from './calcDiff';

const monthly = (pods, budgets, bookings, start) => {
    const today = new Date();
    let date_old = new Date(2000, 0, 1);
    let date = start;
    let cumulated = cumulate(pods, budgets, bookings, date_old, date_old);
    let result = [];
    do {
        cumulated = cumulate(pods, budgets, bookings, date_old, date, cumulated);
        result.push({
            pods: {...cumulated.pod},
            budgets: {...cumulated.budget},
            overview: {
                ...cumulatePods(cumulated, pods), 
                incomming: cumulated.incomming,
                outgoing: cumulated.outgoing,
            },
            date: new Date(date.getTime()),
        });
        date_old = new Date(date.getTime())
        date.setMonth(date.getMonth()+1)
    } while (date.getTime() <= today.getTime());
    const t = new Date(today.getTime());
    t.setDate(t.getDate() +1);
    cumulated = cumulate(pods, budgets, bookings, date_old, today, cumulated);
    result.push({
        pods: {...cumulated.pod},
        budgets: {...cumulated.budget},
        budget_sum: cumulateBudgets(cumulated, budgets),
        overview: {
            ...cumulatePods(cumulated, pods), 
            incomming: cumulated.incomming,
            outgoing: cumulated.outgoing,
        },
        date: new Date(today.getTime()),
    });
    result = calcDiff(result);
    cumulateYear(result);
    return result;
};

export default monthly;