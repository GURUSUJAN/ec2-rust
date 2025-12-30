document.addEventListener('DOMContentLoaded', () => {
    // We fetch from /api, which Nginx will proxy to the backend container
    fetch('/api/data')
        .then(response => response.json())
        .then(data => {
            updateDashboard(data);
        })
        .catch(error => {
            console.error('Error fetching banking data:', error);
            document.querySelector('.status-badge').innerHTML = "● Connection Failed";
            document.querySelector('.status-badge').style.color = "#ef4444";
        });
});

function updateDashboard(data) {
    document.getElementById('user-name').innerText = data.holder;
    document.getElementById('balance').innerText = data.balance.toLocaleString();
    document.getElementById('income').innerText = data.income.toLocaleString();
    document.getElementById('expense').innerText = data.expense.toLocaleString();
    document.getElementById('card-last4').innerText = data.account_number;

    const txList = document.getElementById('tx-list');
    txList.innerHTML = ''; // Clear loading text

    data.transactions.forEach(tx => {
        const li = document.createElement('li');
        li.className = 'tx-item';
        li.innerHTML = `
            <div class="tx-info">
                <span class="tx-merchant">${tx.merchant}</span>
                <span class="tx-cat">${tx.category}</span>
            </div>
            <span class="tx-amount">-$${tx.amount.toFixed(2)}</span>
        `;
        txList.appendChild(li);
    });
}