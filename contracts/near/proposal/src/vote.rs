





/*



Coded by



 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██░
░ ▓░▒ ▒  ░▓  ░ ▒░▓  ░ ▒▒▓  ▒ ░ ▒░▒░▒░ ░ ▒░   ▒ ▒ ░▓  ░ ▒░▒░▒░ ░ ▒░   ▒ ▒ 
  ▒ ░ ░   ▒ ░░ ░ ▒  ░ ░ ▒  ▒   ░ ▒ ▒░ ░ ░░   ░ ▒░ ▒ ░  ░ ▒ ▒░ ░ ░░   ░ ▒░
  ░   ░   ▒ ░  ░ ░    ░ ░  ░ ░ ░ ░ ▒     ░   ░ ░  ▒ ░░ ░ ░ ▒     ░   ░ ░ 
    ░     ░      ░  ░   ░        ░ ░           ░  ░      ░ ░           ░ 
                      ░                                                  



*/





use crate::*; // loading all defined crates, structs and functions from the root crate which is lib.rs in our case






#[near_bindgen]
impl ProposalContract{

    //// vote on a specific proposal
    #[payable]
    pub fn vote(&mut self, proposal_id: ProposalId, voter: Voter) -> Option<Proposal>{

        assert_one_yocto(); //// voter must deposit 1 yocto near
        if let Some(mut proposal) = self.proposals_by_id.get(&proposal_id){

            let mut upvotes = proposal.upvotes.unwrap();
            let mut downvotes = proposal.downvotes.unwrap();

            if voter.is_upvote{
              upvotes+=1;
            } else{
              downvotes+=1;
            }


            let mut voters = proposal.voters;
            let index = voters.iter().position(|v| v.nft_owner_id == voter.nft_owner_id); //-- this owner has alreay voted to this event
            if index == None{
                voters.push(voter);
            }
            proposal.upvotes = Some(upvotes);
            proposal.downvotes = Some(downvotes);
            proposal.voters = voters;
            self.proposals_by_id.insert(&proposal_id, &proposal);
            Some(proposal)


        } else{
          env::log_str("SMARTIES : Proposal With This Id Is Not Exist");
          None
        }



    }




}