const offByOne = d => {
    let r = new Date(d.getTime());
    r.setDate(r.getDate() - 1);
    return r;
};


const cumulateYear = monthly => {
    const firstYear = monthly[0].date.getFullYear();
    const lastYear = monthly[monthly.length -1].date.getFullYear();

    let yearly = {};

    for (let year = firstYear+0; year < lastYear+1; ++year) {
        const incomming = monthly.filter(month => offByOne(month.date).getFullYear() == year).map(month => month.overview.incomming).reduce((a, b) => a + b);
        const outgoing = monthly.filter(month => offByOne(month.date).getFullYear() == year).map(month => month.overview.outgoing).reduce((a, b) => a + b);
        const real_diff = monthly.filter(month => offByOne(month.date).getFullYear() == year).map(month => month.overview.real_diff).reduce((a, b) => a + b);
        yearly[year] = {incomming, outgoing, real_diff, year};
    }

    monthly.forEach(month => {
        if (month.date.getMonth() === 0) {
            month.lastYear = yearly[month.date.getFullYear() -1];
        }
    });
    monthly[monthly.length -1].lastYear = yearly[monthly[monthly.length -1].date.getFullYear()];
};

export default cumulateYear;