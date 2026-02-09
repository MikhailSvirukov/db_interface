document.addEventListener('DOMContentLoaded', () => {
    const loginForm = document.getElementById('loginForm');
    const dashboard = document.getElementById('dashboard');
    const loginContainer = document.querySelector('.login-container');
    const errorMessage = document.getElementById('errorMessage');
    const tablesContainer = document.getElementById('tablesContainer');

    let userCredentials = null;
    let userAccessLevel = null;

    loginForm.addEventListener('submit', async (event) => {
        event.preventDefault();

        const login = document.getElementById('login').value;
        const password = document.getElementById('password').value;

        try {
            const response = await fetch('http://127.0.0.1:3000/login', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    credentials: {
                        login: login,
                        password: password,
                    },
                    payload: null,
                }),
            });

            if (response.ok) {
                const data = await response.json();
                userCredentials = data.credentials;
                userAccessLevel = data.credentials.access_level;
                renderDashboard(data.payload);
                loginContainer.style.display = 'none';
                dashboard.style.display = 'block';
            } else {
                errorMessage.textContent = 'Login failed. Please check your credentials.';
            }
        } catch (error) {
            console.error('Error during login:', error);
            errorMessage.textContent = 'An error occurred. Please try again later.';
        }
    });

    function renderDashboard(payload) {
        tablesContainer.innerHTML = ''; // Clear previous content

        for (const tableName in payload) {
            const tableData = payload[tableName];
            if (Array.isArray(tableData) && tableData.length > 0) {
                const tableSection = document.createElement('div');
                tableSection.className = 'table-section';

                const tableTitle = document.createElement('h3');
                tableTitle.textContent = tableName.charAt(0).toUpperCase() + tableName.slice(1);
                tableSection.appendChild(tableTitle);

                const tableElement = document.createElement('table');
                const thead = document.createElement('thead');
                const tbody = document.createElement('tbody');

                // Create table headers
                const headers = Object.keys(tableData[0]);
                const headerRow = document.createElement('tr');
                headers.forEach(headerText => {
                    const th = document.createElement('th');
                    th.textContent = headerText.charAt(0).toUpperCase() + headerText.slice(1);
                    headerRow.appendChild(th);
                });
                thead.appendChild(headerRow);
                tableElement.appendChild(thead);

                // Create table rows
                tableData.forEach(rowData => {
                    const tr = document.createElement('tr');
                    headers.forEach(headerText => {
                        const td = document.createElement('td');
                        td.textContent = rowData[headerText];
                        tr.appendChild(td);
                    });
                    tbody.appendChild(tr);
                });
                tableElement.appendChild(tbody);
                tableSection.appendChild(tableElement);
                tablesContainer.appendChild(tableSection);

                // Add forms for adding and updating items if the user has appropriate access
                if (userAccessLevel === 'Administrator' || userAccessLevel === 'Programmer') {
                    addUpdateForms(tableName, headers, tableSection);
                }
            }
        }
    }

    function addUpdateForms(tableName, headers, tableSection) {
        const formattedTableName = tableName.slice(0, -1); // e.g., sections -> section

        // Add New Item Form
        const addForm = document.createElement('div');
        addForm.className = 'add-item-form';
        addForm.innerHTML = `
            <h4>Add New ${formattedTableName.charAt(0).toUpperCase() + formattedTableName.slice(1)}</h4>
            <form id="add${tableName}Form">
                ${headers.map(header => `
                    <div>
                        <label for="add-${formattedTableName}-${header}">${header.charAt(0).toUpperCase() + header.slice(1)}:</label>
                        <input type="text" id="add-${formattedTableName}-${header}" name="${header}" ${header === 'id' ? 'readonly' : ''} required>
                    </div>
                `).join('')}
                <button type="submit">Add ${formattedTableName.charAt(0).toUpperCase() + formattedTableName.slice(1)}</button>
            </form>
        `;
        tableSection.appendChild(addForm);

        document.getElementById(`add${tableName}Form`).addEventListener('submit', (event) => handleAddItem(event, tableName, headers));

        // Update Item Form
        const updateForm = document.createElement('div');
        updateForm.className = 'update-item-form';
        updateForm.innerHTML = `
            <h4>Update ${formattedTableName.charAt(0).toUpperCase() + formattedTableName.slice(1)}</h4>
            <form id="update${tableName}Form">
                ${headers.map(header => `
                    <div>
                        <label for="update-${formattedTableName}-${header}">${header.charAt(0).toUpperCase() + header.slice(1)}:</label>
                        <input type="text" id="update-${formattedTableName}-${header}" name="${header}" required>
                    </div>
                `).join('')}
                <button type="submit">Update ${formattedTableName.charAt(0).toUpperCase() + formattedTableName.slice(1)}</button>
            </form>
        `;
        tableSection.appendChild(updateForm);

        document.getElementById(`update${tableName}Form`).addEventListener('submit', (event) => handleUpdateItem(event, tableName, headers));
    }

    async function handleAddItem(event, tableName, headers) {
        event.preventDefault();
        const form = event.target;
        const payload = {};
        headers.forEach(header => {
            payload[header] = form.elements[header].value;
        });

        const endpoint = `http://127.0.0.1:3000/${tableName}/add`;
        await sendDataToServer(endpoint, payload);
    }

    async function handleUpdateItem(event, tableName, headers) {
        event.preventDefault();
        const form = event.target;
        const payload = {};
        headers.forEach(header => {
            payload[header] = form.elements[header].value;
        });

        const endpoint = `http://127.0.0.1:3000/${tableName}/update`;
        await sendDataToServer(endpoint, payload);
    }

    async function sendDataToServer(endpoint, payload) {
        try {
            const response = await fetch(endpoint, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    credentials: userCredentials,
                    payload: payload,
                }),
            });

            if (response.ok) {
                // Re-fetch all data to refresh the dashboard
                const refreshResponse = await fetch('http://127.0.0.1:3000/login', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({
                        credentials: userCredentials,
                        payload: null,
                    }),
                });

                if (refreshResponse.ok) {
                    const data = await refreshResponse.json();
                    renderDashboard(data.payload);
                    errorMessage.textContent = '';
                } else {
                    errorMessage.textContent = 'Failed to refresh data after update.';
                }
            } else {
                const errorText = await response.text();
                errorMessage.textContent = `Operation failed: ${errorText}`;
            }
        } catch (error) {
            console.error('Error during data operation:', error);
            errorMessage.textContent = 'An error occurred during data operation.';
        }
    }
});