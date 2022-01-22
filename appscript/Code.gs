function doPost(req) {
  const { postData: { contents, type } = {} } = req;
  const { method, username, password, data } = JSON.parse(contents);

  if (!(username == '<USERNAME>' && password == '<PASSWORD>')){
    return ContentService.createTextOutput(JSON.stringify({'status': 401})).setMimeType(ContentService.MimeType.JSON);
  }

  switch (method) {
    case 'addBooking':
      return ContentService.createTextOutput(JSON.stringify({'status': addBooking(data)})).setMimeType(ContentService.MimeType.JSON);
    case 'getBookings':
      return ContentService.createTextOutput(JSON.stringify({'data': getBookings(), 'status': 200})).setMimeType(ContentService.MimeType.JSON);
    case 'addBudget':
      return ContentService.createTextOutput(JSON.stringify({'status': addBudget(data)})).setMimeType(ContentService.MimeType.JSON);
    case 'getBudgets':
      return ContentService.createTextOutput(JSON.stringify({'data': getBudgets(), 'status': 200})).setMimeType(ContentService.MimeType.JSON);
    case 'addPod':
      return ContentService.createTextOutput(JSON.stringify({'status': addPod(data)})).setMimeType(ContentService.MimeType.JSON);
    case 'getPods':
      return ContentService.createTextOutput(JSON.stringify({'data': getPods(), 'status': 200})).setMimeType(ContentService.MimeType.JSON);
    default:
      return ContentService.createTextOutput(JSON.stringify({'status': 404})).setMimeType(ContentService.MimeType.JSON);
  }
}

function getBookings() {
  const bookings = SpreadsheetApp.openById('<deployment-id>').getSheetByName('Bookings').getDataRange().getValues().map(line => ({
    'date': line[0],
    'amount': line[1],
    'message': line[2],
    'from_pod': line[3],
    'from_budget': line[4],
    'to_pod': line[5],
    'to_budget': line[6],
  }));
  bookings.shift();
  return bookings;
}

function addBooking(booking) {
  if (!('date' in booking && 'amount' in booking && 'message' in booking && 'from_pod' in booking && 'from_budget' in booking && 'to_pod' in booking && 'to_budget' in booking)) {
    return 406;
  }

  SpreadsheetApp.openById('<deployment-id>').getSheetByName('Bookings').appendRow([
    booking.date,
    booking.amount,
    booking.message,
    booking.from_pod,
    booking.from_budget,
    booking.to_pod,
    booking.to_budget,
  ]);

  return 201;
}

function getBudgets() {
  const budgets = SpreadsheetApp.openById('<deployment-id>').getSheetByName('Budgets').getDataRange().getValues().map(line => ({
    'name': line[0],
    'monthly': line[1],
  }));
  budgets.shift();
  return budgets;
}

function addBudget(budget) {
  if (!('name' in budget && 'monthly' in budget)) {
    return 406;
  }

  SpreadsheetApp.openById('<deployment-id>').getSheetByName('Budgets').appendRow([
    budget.name, 
    budget.monthly,
  ]);

  return 201;
}

function getPods() {
  const pods = SpreadsheetApp.openById('<deployment-id>').getSheetByName('Pods').getDataRange().getValues().map(line => ({
    'name': line[0],
    'type': line[1],
  }));
  pods.shift();
  return pods;
}

function addPod(pod) {
  if (!('name' in pod && 'type' in pod)) {
    return 406;
  }

  SpreadsheetApp.openById('<deployment-id>').getSheetByName('Pods').appendRow([
    pod.name, pod.type
  ]);

  return 201;
}