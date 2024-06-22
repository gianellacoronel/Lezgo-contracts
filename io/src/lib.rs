#![no_std]

use gmeta::{In, InOut, Metadata, Out};
use gstd::{ActorId, prelude::*};

pub struct ProgramMetadata;

// 1. Define actions, events, errors and state for your metadata.
impl Metadata for ProgramMetadata {
    type Init = In<InitStruct>;
    type Handle = InOut<ConcertActions, Result<ConcertEvents, ConcertErrors>>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = InOut<ConcertQuery, ConcertQueryReply>;
}

// 2. Create your init Struct(Optional)
#[derive(Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitStruct {
    // Example:
    pub ft_program_id: ActorId,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct CreateConcertProps {
    pub creator: ActorId,
    pub name: String,
    pub description: String,
    pub number_of_tickets: u128,
    pub token_id: u128,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct BuyTicketsProps {
    pub amount: u128,
}

// 3. Create your own Actions
#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ConcertActions {
    // Actions
    Create(CreateConcertProps),
    Hold,
    BuyTickets(BuyTicketsProps),
}

// Add Your Main State
#[derive(Debug, Decode, Encode, Clone, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct ConcertState {
    pub name: String,
    pub description: String,
    pub number_of_tickets: u128,
    pub creator: ActorId,
    pub owner_id: ActorId,
}

// 4. Create your own Events
#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ConcertEvents {
    // Events
    Creation {
        creator: ActorId,
        concert_id: u128,
        number_of_tickets: u128,
    },
    Hold {
        concert_id: u128,
    },
    Purchase {
        concert_id: u128,
        amount: u128,
    },
}

// 5. Create your own Errors
#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ConcertErrors {
    AlreadyRegistered,
    ZeroAddress,
    LessThanOneTicket,
    NotEnoughTickets,
    NotEnoughMetadata,
    NotCreator,
}

// 6. Create your own Struct
#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct ConcertCustomFields {
    pub name: String,
    pub description: String,
}

// 7. Create your own enum
#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum CustomStates {
    // Add your states
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ConcertQuery {
    ConcertName,
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ConcertQueryReply {
    ConcertName(String),
}
