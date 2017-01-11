An essential part of the client/server protocol in haret is the **ClientID**.

A ClientID is an opaque string (uuid is recommended) provided by a
client after connecting to a haret server in order to identify
itself. Knowledge of currently connected and recently connected
ClientIDs is expected to be shared globally across the entire haret
cluster so that all servers will be aware of all clients.

**Continuity**: a given instance of running client software is identified
by its ClientID for the entire duration of a connection. That same
running client instance may choose to continue using the same ClientID
if it is disconnected and later reconnects, if the client side has
continued to run without local interruption.

**Uniqueness**: a client is responsible for creating a ClientID that
should not collide with that of any other current or recent
client. When a haret server is sent a conflicting (already or recently
in use) ClientID by a client, the server may send any or all such
clients a “who are you?” message to see if they are live. The server
will disconnect clients as needed in order to ensure that no more than
one client is currently using a given ClientID.

**Purpose**: a primary reason for ClientIDs is to enable idempotent /
replayable messages for clients, even across a disconnection. In the
interest of this purpose, a haret server will tell a client whether its
ClientID is “new” or “continued” after receiving that ClientID so that
the client in question can know whether it can take advantage of
idempotency from its prior connection.

**Expiry**: in order to make ClientIDs work across connections, haret will
maintain a cluster-wide memory not only of active ClientIDs but also
of recently disconnected ones. The cluster will expire these and
forget about them at its discretion, likely after some fixed number of
minutes since the disconnection. The possibility of this expiry is why
a client may reconnect after some time and yet possibly be told that
its ClientID is “new."

A note on **subscriptions**: despite the ability of a ClientID to span
connections, a client should expect all of its running subscriptions
to data changes to be broken when a connection breaks. If that client
wishes to retain the subscriptions, it will need to re-request them.

