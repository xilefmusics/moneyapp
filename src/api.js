const apiUrl = localStorage.getItem('url');
const username = localStorage.getItem('user');
const password = localStorage.getItem('password');

const createRequest = async (method, data) => {
  const res = await(await fetch(apiUrl, {
    method: 'POST',
    redirect: 'follow',
    headers: {
      'Content-Type': 'text/plain;charset=utf-8',
    },
    body: JSON.stringify({
      username: username,
      password: password,
      method: method,
      data: data,
    }),
  })).json();
  console.log({
    method: method,
    req_data: data,
    status: res.status,
    res_data: res.data,
  })
  return res.data;
}

const convertBookings = bookings => bookings.map(booking => {
  booking.date = new Date(booking.date);
  return booking;
});

export default {
  getPods: async () => createRequest('getPods'),
  addPod: async pod => createRequest('addPod', pod),
  getBudgets: async () => createRequest('getBudgets'),
  addBudget: async budget => createRequest('addBudget', budget),
  getBookings: async () => convertBookings(await createRequest('getBookings')),
  addBooking: async booking => createRequest('addBooking', booking),
};
