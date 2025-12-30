async function getBalance() {
  const res = await fetch("http://localhost:8080/balance/123");
  const data = await res.json();
  document.getElementById("balance").innerText = "Balance: $" + data.balance;
}
