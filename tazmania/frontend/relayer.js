import axios from 'axios';

export async function get_commitments(relayer){
	var res =  await axios.get(relayer + "/commitments");
	return res.data.commitments;
}

export async function send_proof(relayer, public_signals, proof){

	var data = JSON.stringify({
		'public_signals': public_signals,
		'proof': proof
	})

	var config = {
		method: 'post',
		url: relayer + "/withdraw",
		headers: {
		    'Content-Type': 'application/json'
		},
		data: data
	}

	var res = await axios(config)
	console.log(res);
	return res.data;
}

