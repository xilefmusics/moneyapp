use crate::error::AppError;
use money_app_shared::transaction::Transaction;

use super::{Barclays, N26new, N26};

pub fn import(s: &str) -> Result<Vec<Transaction>, AppError> {
    Barclays::parse_transactions(s)
        .or_else(|_| N26new::parse_transactions(s))
        .or_else(|_| N26::parse_transactions(s))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n26() {
        let input = "\"Datum\",\"Empfänger\",\"Kontonummer\",\"Transaktionstyp\",\"Verwendungszweck\",\"Betrag (EUR)\",\"Betrag (Fremdwährung)\",\"Fremdwährung\",\"Wechselkurs\"
\"2023-12-01\",\"Receiver 1\",\"DE123...456\",\"Überweisung\",\"Transaction 1\",\"-123.4\",\"\",\"\",\"\"
\"2023-12-02\",\"Receiver 2\",\"DE123...456\",\"Lastschrift\",\"Transaction 2\",\"-8.99\",\"\",\"\",\"\"
";
        println!("{:?}", import(input));
    }

    #[test]
    fn test_barclays() {
        let input = "
,,,,,,,,,,,,,
Transaktionsansicht,,,,,,,,,,,,,
,,,,,,,,,,,,,
Kontonummer,123456,,,,,,,,,,,,
IBAN,DE123456,,,,,,,,,,,,
Kontoname,Barclays Visa,,,,,,,,,,,,
Kontotyp,Kreditkarte,,,,,,,,,,,,
Kreditrahmen,\"1.2345,00\",,,,,,,,,,,,
Verfügungsrahmen,\"1.000,00\",,,,,,,,,,,,
Status,Aktiv,,,,,,,,,,,,
Stand,23 Dez 23,,,,,,,,,,,,
,,,,,,,,,,,,,
Referenznummer,Buchungsdatum,Buchungsdatum,Betrag,Beschreibung,Typ,Status,Kartennummer,Originalbetrag,Mögliche Zahlpläne,Land,Name des Karteninhabers,Kartennetzwerk,Kontaktlose Bezahlung
123456,01.12.2023,,\"+1,23 €\",Transaction 1,Überweisung,vorgemerkt,9876,\"+1,23 €\",Nein,DE,Name,Visa,Ja
123457,02.12.2023,03.12.2023,\"-1,23 €\",Transaction2,Belastung,noch nicht abgerechnet,5432,\"-1,23 €\",Nein,DE,Name,Visa,Nein
";
        println!("{:?}", import(input));
    }

    #[test]
    fn test_n26new() {
        let input = r#""Booking Date","Value Date","Partner Name","Partner Iban","Type","Payment Reference","Account Name","Amount (EUR)","Original Amount","Original Currency","Exchange Rate"
"2024-10-01","2024-10-01","Partner A","DEXXXXXXXXXXXXXXXXXX","Debit Transfer","Rent October 2024","Main Account",-980.00,,,
"2024-10-10","2024-10-10","Telecom Provider","DEXXXXXXXXXXXXXXXXXX","Direct Debit","Reference A","Main Account",-8.99,,,
"2024-10-14","2024-10-14","Utility Company","DEXXXXXXXXXXXXXXXXXX","Direct Debit","Reference B","Main Account",-44.90,,,
"2024-10-15","2024-10-15","Broadcasting Service","DEXXXXXXXXXXXXXXXXXX","Direct Debit","Broadcast Contribution","Main Account",-55.08,,,
"2024-10-17","2024-10-17","Mortgage Bank","DEXXXXXXXXXXXXXXXXXX","Direct Debit","Reference C","Main Account",-50.00,,,
"2024-10-24","2024-10-24","Energy Provider","DEXXXXXXXXXXXXXXXXXX","Direct Debit","Direct-Debit","Main Account",-46.08,,,
"2024-10-28","2024-10-28","Employer","DEXXXXXXXXXXXXXXXXXX","Credit Transfer","Salary October","Main Account",2918.88,,,
"2024-10-31","2024-10-31","Credit Card Provider","DEXXXXXXXXXXXXXXXXXX","Direct Debit","Card Payment","Main Account",-2000.00,,,
"2024-11-01","2024-11-01","Partner A","DEXXXXXXXXXXXXXXXXXX","Debit Transfer","Rent November 2024","Main Account",-980.00,,,
"2024-11-01","2024-11-01","Payment Service","LUXXXXXXXXXXXXXXXXXX","Direct Debit","Subscription A","Main Account",-474.00,,,
"2024-11-04","2024-11-04","Private Transfer","DEXXXXXXXXXXXXXXXXXX","Credit Transfer","","Main Account",600.00,,,
"2024-11-04","2024-11-04","Payment Service","LUXXXXXXXXXXXXXXXXXX","Direct Debit","Subscription B","Main Account",-4.99,,,
"2024-11-05","2024-11-05","Online Store","DEXXXXXXXXXXXXXXXXXX","Direct Debit","Order 1","Main Account",-31.83,,,
"2024-11-05","2024-11-05","Payment Service","LUXXXXXXXXXXXXXXXXXX","Direct Debit","Subscription C","Main Account",-37.99,,,
"2024-11-05","2024-11-05","Payment Service","LUXXXXXXXXXXXXXXXXXX","Direct Debit","Service Payment","Main Account",-5.99,,,
"2024-11-05","2024-11-05","Payment Service","LUXXXXXXXXXXXXXXXXXX","Direct Debit","Subscription D","Main Account",-34.99,,,
"2024-11-06","2024-11-06","Online Payments","DEXXXXXXXXXXXXXXXXXX","Direct Debit","Order 2","Main Account",-5.99,,,
"2024-11-08","2024-11-08","Telecom Provider","DEXXXXXXXXXXXXXXXXXX","Direct Debit","Reference D","Main Account",-8.99,,,
"2024-11-09","2024-11-08","Retail Store","N/A","Presentment","","Main Account",-61.86,61.86,"EUR",1.0
"#;
        println!("{:?}", import(input));
    }
}
