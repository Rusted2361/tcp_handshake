# tcp_handshake

I have written this script in which we will run a p2p network using substrate node template.

**Steps**
First we need to install rust in our system by runnig the following command.

**curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh**

Once the installation is complete, the installer will provide instructions on how to set up the Rust environment. 
Typically, you'll need to add the Rust binaries to your system's PATH variable. This allows you to run Rust commands from any terminal window.
To verify that Rust is installed run following command

**rustc --version**

Once you are done with this need to setup a p2p network.

lets follow the command one by one

**git clone git@github.com:Rusted2361/tcp_handshake.git**

**cd substrate-node-template**

**cargo build --release**

**./target/release/node-template --dev**


The system will serve at local host on port 9944. This will start a local p2p dev network on your system that you can access on this link.

**https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer**

After  setting up environment & running p2p network we need to test or tcp_handshake script.

we will simply go to the root directory and run command

**cargo run 
**

It will show you following message.

Substrate handshake successful. Remote version: 791752241

Here is the end of demonstration reach me out on my linkedin for further queries.
https://www.linkedin.com/in/hamza-sajid-3b6122174/
