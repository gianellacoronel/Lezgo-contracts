#![no_std]

use gstd::{ActorId, async_main, msg, prelude::*};

use io::*;

const ZERO_ID: ActorId = ActorId::zero();
static mut STATE: Option<Concert> = None;

// Create a public State
#[derive(Clone, Default)]
pub struct Concert {
    pub name: String,
    pub description: String,
    pub number_of_tickets: u128,
    pub creator: ActorId,
    pub owner_id: ActorId,
    pub concert_id: u128,
}

// Create a implementation on State
impl Concert {
    async fn create_concert(
        &mut self,
        input: CreateConcertProps,
    ) -> Result<ConcertEvents, ConcertErrors> {
        // Add Validations

        // Add your logic for example

        // Change State
        self.name = input.name;

        Ok(ConcertEvents::Creation {
            creator: self.creator,
            concert_id: self.concert_id,
            number_of_tickets: self.number_of_tickets,
        })
    }

    async fn hold_concert(&mut self) -> Result<ConcertEvents, ConcertErrors> {
        // Add Validations

        // Add your logic for example

        // Change State

        Ok(ConcertEvents::Hold {
            concert_id: self.concert_id,
        })
    }
    async fn buy_tickets(
        &mut self,
        input: BuyTicketsProps,
    ) -> Result<ConcertEvents, ConcertErrors> {
        // Add Validations

        // Add your logic for example

        // Change State
        self.number_of_tickets = input.amount;

        Ok(ConcertEvents::Purchase {
            concert_id: self.concert_id,
            amount: self.number_of_tickets,
        })
    }
}

// 3. Create the init() function of your contract.
#[no_mangle]
extern "C" fn init() {
    let state = Concert {
        ..Default::default()
    };

    unsafe { STATE = Some(state) };
}

// 4.Create the main() function of your contract.
#[async_main]
async fn main() {
    // We load the input message
    let action: ConcertActions = msg::load().expect("Could not load Action");

    let state = unsafe { STATE.as_mut().expect("The contract is not initialized") };

    // We receive an action from the user and update the state. Example:

    let reply = match action {
        ConcertActions::Create(input) => state.create_concert(input).await,
        ConcertActions::Hold => state.hold_concert().await,
        ConcertActions::BuyTickets(input) => state.buy_tickets(input).await,
    };
    msg::reply(reply, 0).expect("Error in sending a reply");
}

// 5. Create the state() function of your contract.
#[no_mangle]
extern "C" fn state() {
    let state = unsafe { STATE.take().expect("Unexpected error in taking state") };
    let query: ConcertQuery = msg::load().expect("Unable to load the state query");
    let reply = match query {
        ConcertQuery::ConcertName => ConcertQueryReply::ConcertName(state.name),
    };
    msg::reply(reply, 0).expect("Error on state");
}
