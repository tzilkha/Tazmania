const { Pool, Client } = require("pg");
const { explorer_host, 
	explorer_database, 
	explorer_user, 
	explorer_password,
	tazmania_id } = require('./config')

const postgreSQL_select_Query = "select t.block_timestamp, r.predecessor_account_id, e.status, ara.args " +
		"from receipts r, action_receipt_actions ara, transactions t, execution_outcomes e " +
		"where r.receiver_account_id ='" + tazmania_id + "' " +
  		"and ara.receipt_id = r.receipt_id " +
  		"and r.receipt_id = e.receipt_id " +
  		"and ara.action_kind = 'FUNCTION_CALL' " +
  		"and ara.args @> '{\"method_name\": \"deposit\"}' " +
  		"and t.transaction_hash = r.originated_from_transaction_hash " +
		"order by t.block_timestamp";

const credentials = {
  user: explorer_user,
  host: explorer_host,
  database: explorer_database,
  password: explorer_password,
  port: 5432,
};

// Connect with a connection pool.

async function get_commitments(req, res) {
  const pool = new Pool(credentials);
  const q_res = await pool.query(postgreSQL_select_Query);
  await pool.end();

  commitments = q_res.rows.map((el) => {return el.args.args_json.commitment})

  res.json({'commitments': commitments});
}

module.exports = {get_commitments}
