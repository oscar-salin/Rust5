fn main() {

    let package1 = ParcelStatus::Preparing;
    let package2 = ParcelStatus::InTransit {
        courier: String::from("Posti")
    };
    let package3 = ParcelStatus::Delivered {
        recipient: String::from("Oscar")
    };
    let package4 = ParcelStatus::Returned { 
        reason: String::from("Broken") 
    };

    println!("{}", status_message(package1));
    println!("{}", status_message(package2));
    println!("{}", status_message(package3));
    println!("{}", status_message(package4));


    print_delivery_note(Some("Leave at reception."));
    print_delivery_note(None);

    let undelivered = ParcelStatus::Preparing;
    let delivered = ParcelStatus::Delivered {recipient: String::from("Oscar")};

    announce_delivery(&delivered);
    announce_delivery(&undelivered);



    create_receipt(delivered);
    create_receipt(undelivered);

}

enum ParcelStatus {
    Preparing,
    InTransit { courier: String },
    Delivered { recipient: String },
    Returned { reason: String },
}

fn status_message(status: ParcelStatus) -> String {
    match status {
        ParcelStatus::Preparing => String::from("Your parcel is being prepared."),
        ParcelStatus::InTransit { courier } => format!("Your parcel is in transit with {courier}."),
        ParcelStatus::Delivered { recipient } => format!("Your parcel has been delivered to {recipient}."),
        ParcelStatus::Returned { reason } => format!("Your parcel has been returned. Reason: {reason}."),
    }
}

fn print_delivery_note(note: Option<&str>) {
    if let Some(note) = note {
        println!("Delivery Note: {note}");
    } else {
        println!("No delivery note provided.");
    }
}

fn announce_delivery(status: &ParcelStatus) {
    if let ParcelStatus::Delivered { recipient } = status {
        println!("Parcel was delivered to {recipient}");
    } else {
        println!("Parcel has not been delivered yet.");
    }
}

fn create_receipt(status: ParcelStatus) {
    let ParcelStatus::Delivered { recipient } = status else {
        println!("Parcel has not been delivered yet.");
        return;
    };

    println!("Receipt created for {recipient}.");
}
