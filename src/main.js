document.addEventListener('DOMContentLoaded', async () => {
  const serialPortsSelect = document.getElementById('serialPorts');
  const connectBtn = document.getElementById('connectBtn');
  const powerOnBtn = document.getElementById('powerOn');
  const powerOffBtn = document.getElementById('powerOff');
  const hdmi1Btn = document.getElementById('hdmi1');
  const hdmi2Btn = document.getElementById('hdmi2');
  const connectionStatus = document.getElementById('connectionStatus');
  const logDiv = document.getElementById('log');

  // Load available serial ports
  async function loadSerialPorts() {
      try {
          const ports = await window.__TAURI_INVOKE__('get_serial_ports');
          serialPortsSelect.innerHTML = '';
          ports.forEach(port => {
              const option = document.createElement('option');
              option.value = port.port_name;
              option.textContent = port.port_name;
              serialPortsSelect.appendChild(option);
          });
          log('Loaded serial ports');
      } catch (error) {
          log(`Error loading ports: ${error}`);
      }
  }

  // Connect to selected port
  connectBtn.addEventListener('click', async () => {
      const port = serialPortsSelect.value;
      if (!port) {
          log('Please select a serial port');
          return;
      }

      try {
          await window.__TAURI_INVOKE__('connect_projector', { portName: port });
          connectionStatus.textContent = `Connected to ${port}`;
          connectionStatus.style.color = 'green';
          log(`Connected to ${port}`);
      } catch (error) {
          connectionStatus.textContent = 'Connection failed';
          connectionStatus.style.color = 'red';
          log(`Connection error: ${error}`);
      }
  });

  // Projector control functions
  async function sendCommand(command, description) {
      try {
          const response = await window.__TAURI_INVOKE__('send_command', { command });
          log(`${description}: ${response}`);
      } catch (error) {
          log(`Error (${description}): ${error}`);
      }
  }

  // Button event listeners
  powerOnBtn.addEventListener('click', () => sendCommand('~00PON', 'Power On'));
  powerOffBtn.addEventListener('click', () => sendCommand('~00POF', 'Power Off'));
  hdmi1Btn.addEventListener('click', () => sendCommand('~00SIN01', 'HDMI 1'));
  hdmi2Btn.addEventListener('click', () => sendCommand('~00SIN02', 'HDMI 2'));

  // Log messages to UI
  function log(message) {
      const entry = document.createElement('div');
      entry.textContent = `[${new Date().toLocaleTimeString()}] ${message}`;
      logDiv.appendChild(entry);
      logDiv.scrollTop = logDiv.scrollHeight;
  }

  // Initialize
  await loadSerialPorts();
  log('Application started');
});
