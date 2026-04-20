#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol, log};

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {
    /// Memberikan suara untuk kandidat tertentu.
    /// Memeriksa apakah pemilih sudah pernah memberikan suara sebelumnya.
    pub fn vote(env: Env, voter: Address, candidate: Symbol) {
        // Autentikasi: Memastikan pengirim adalah benar pemilik Address ini
        voter.require_auth();

        // Cek apakah user sudah pernah vote (Persistent Storage)
        if env.storage().persistent().has(&voter) {
            log!(&env, "User {} sudah memberikan suara!", voter);
            panic!("Sudah pernah memberikan suara!");
        }

        // Ambil jumlah suara saat ini untuk kandidat (Instance Storage)
        let mut count: u32 = env.storage().instance().get(&candidate).unwrap_or(0);
        
        // Tambahkan suara
        count += 1;
        
        // Simpan kembali jumlah suara terbaru
        env.storage().instance().set(&candidate, &count);

        // Tandai user sudah melakukan vote
        env.storage().persistent().set(&voter, &true);
        
        log!(&env, "Suara berhasil dicatat untuk: {}", candidate);
    }

    /// Mendapatkan jumlah suara total untuk seorang kandidat.
    pub fn get_votes(env: Env, candidate: Symbol) -> u32 {
        env.storage().instance().get(&candidate).unwrap_or(0)
    }
}

mod test;