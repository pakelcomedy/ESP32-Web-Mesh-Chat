// script.js – Simple Mesh Chat client

(() => {
  const apiBase = ''; // root
  const chatWindow = document.getElementById('chatWindow');
  const chatForm   = document.getElementById('chatForm');
  const senderInput  = document.getElementById('senderInput');
  const messageInput = document.getElementById('messageInput');

  // Load or prompt for username once
  let username = localStorage.getItem('meshChatName') || '';
  if (!username) {
    username = prompt('Enter your display name (max 12 chars):', '')?.slice(0,12) || 'Anon';
    localStorage.setItem('meshChatName', username);
  }
  senderInput.value = username;
  senderInput.readOnly = true;

  // Render a single message
  function renderMessage({ sender, message, timestamp }) {
    const time = new Date(timestamp).toLocaleTimeString().slice(0,5);
    const div = document.createElement('div');
    div.className = sender === username ? 'message sent' : 'message received';
    div.innerHTML = `
      <span class="message__meta">${sender} • ${time}</span>
      <div class="message__text">${message}</div>
    `;
    chatWindow.appendChild(div);
    chatWindow.scrollTop = chatWindow.scrollHeight;
  }

  // Fetch and display all messages
  async function loadMessages() {
    try {
      const res = await fetch(`${apiBase}/messages`);
      if (!res.ok) throw new Error(res.statusText);
      const msgs = await res.json();
      chatWindow.innerHTML = '';
      msgs.forEach(renderMessage);
    } catch (e) {
      console.error('Failed to load messages:', e);
    }
  }

  // Send new message
  chatForm.addEventListener('submit', async e => {
    e.preventDefault();
    const text = messageInput.value.trim();
    if (!text) return;
    const payload = { sender: username, message: text };
    try {
      const res = await fetch(`${apiBase}/send`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
      });
      if (!res.ok) throw new Error(res.statusText);
      messageInput.value = '';
      await loadMessages();
    } catch (e) {
      console.error('Failed to send:', e);
    }
  });

  // Initial load & polling
  loadMessages();
  setInterval(loadMessages, 2000);
})();
