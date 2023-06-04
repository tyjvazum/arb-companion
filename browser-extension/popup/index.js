function listenForClicks() {
  document.addEventListener("click", (e) => {
    function cmdNameToCmd(cmdName) {
      switch (cmdName) {
        case "Find Sat":
            const sat1 = document.querySelector("#find").value;
            return "arb>find/"+sat1;
        case "Epochs":
            return "arb>epochs";
        case "Index":
            return "arb>index";
        case "Info":
            return "arb>info";
        case "List Sat":
            const outpoint = document.querySelector("#list").value;
            return "arb>list/"+outpoint;
        case "Parse Sat":
            const sat2 = document.querySelector("#parse").value;
            return "arb>parse/"+sat2;
        case "Subsidy at Height":
            const height = document.querySelector("#subsidy").value;
            return "arb>subsidy/"+height;
        case "Supply":
            return "arb>supply";
        case "Traits Sat":
            const sat3 = document.querySelector("#traits").value;
            return "arb>traits/"+sat3;
        case "Wallet>Balance":
            return "arb>wallet>balance";
        case "Wallet>Cardinals":
            return "arb>wallet>cardinals";
        case "Wallet>Create":
            return "arb>wallet>create";
        case "Inscribe File":
            const rate1 = document.querySelector("#rate").value;
            const path = document.querySelector("#path").value;
            return "arb>wallet>inscribe/--fee-rate:"+rate1+"/"+path;
        case "Wallet>Inscriptions":
            return "arb>wallet>inscriptions";
        case "Wallet>Outputs":
            return "arb>wallet>outputs";
        case "Wallet>Receive":
            return "arb>wallet>receive";
        case "Restore Wallet":
            const mnemonic = document.querySelector("#mnemonic").value;
            return "arb>wallet>restore/"+mnemonic;
        case "Wallet>Sats":
            return "arb>wallet>sats";
        case "Send":
            const rate2 = document.querySelector("#rate").value;
            const address = document.querySelector("#address").value;
            const outgoing = document.querySelector("#outgoing").value;
            return "arb>wallet>send/--fee-rate:"+rate2+"/"+address+"/"+outgoing;
        case "Wallet>Transactions":
            return "arb>wallet>transactions";
        default:
            return false;
      }
    }

    if (e.target.tagName !== "BUTTON" || !e.target.closest("#popup-content")) {
      // Ignore when click is not on a button within <div id="popup-content">.
      return;
    }
    const cmd = cmdNameToCmd(e.target.textContent);
    if (cmd) {
        let port = browser.runtime.connectNative("arb_companion");
        port.onMessage.addListener((response) => {
            if (isJsonString(response.payload)) {
                console.log(JSON.parse(response.payload));
                document.querySelector("#popup-output").innerText = response.payload;
            } else {
                console.log("Received: " + JSON.stringify(response));
                document.querySelector("#popup-output").innerText = response.payload;
            }
        });
        port.postMessage(cmd);
    } else {
        const str = e.target.textContent;
        if (str == "Find") {
            document.querySelector("#popup-output").innerHTML = `
                <input id="find" class="input-field"></input>
                <button class="dark-button">Find Sat</button>
            `;
        } else if (str == "List") {
            document.querySelector("#popup-output").innerHTML = `
                <input id="list" class="input-field"></input>
                <button class="dark-button">List Sat</button>
            `;
        } else if (str == "Parse") {
            document.querySelector("#popup-output").innerHTML = `
                <input id="parse" class="input-field"></input>
                <button class="dark-button">Parse Sat</button>
            `;
        } else if (str == "Subsidy") {
            document.querySelector("#popup-output").innerHTML = `
                <input id="subsidy" class="input-field"></input>
                <button class="dark-button">Subsidy at Height</button>
            `;
        } else if (str == "Traits") {
            document.querySelector("#popup-output").innerHTML = `
                <input id="traits" class="input-field"></input>
                <button class="dark-button">Traits Sat</button>
            `;
        } else if (str == "Wallet>Inscribe") {
            document.querySelector("#popup-output").innerHTML = `
                <input id="rate" placeholder="Fee rate in sats" class="input-field"></input>
                <input id="path" placeholder="Path to file to inscribe" class="input-field"></input>
                <button class="dark-button">Inscribe File</button>
            `;
        } else if (str == "Wallet>Restore") {
            document.querySelector("#popup-output").innerHTML = `
                <input id="mnemonic" placeholder="Wallet mnemonic" class="input-field"></input>
                <button class="dark-button">Restore Wallet</button>
            `;
        } else if (str == "Wallet>Send") {
            document.querySelector("#popup-output").innerHTML = `
                <input id="rate" placeholder="Fee rate in sats" class="input-field"></input>
                <input id="address" placeholder="Address to send to" class="input-field"></input>
                <input id="outgoing" placeholder="Sat or inscription to send" class="input-field"></input>
                <button class="dark-button">Send</button>
            `;
        }
    }
  });
}

function isJsonString(str) {
    try {
        JSON.parse(str);
    } catch (e) {
        return false;
    }
    return true;
}

listenForClicks();
