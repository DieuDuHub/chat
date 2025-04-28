import { View, Text } from "react-native";
import React, { useEffect, useState } from "react";

export default function Webs() {
  const [isConnected, setIsConnected] = useState<boolean>(false);
  const [serverMessage, setServerMessage] = useState("");

  useEffect(() => {
    const ws = new WebSocket("ws://127.0.0.1:8000/events");

    ws.onopen = () => {
      console.log("WebSocket connection opened");
      // to send message you can use like that :   ws.send("Hello, server!"); 
      setIsConnected(true); // Update state to reflect successful connection
    };

    ws.onmessage = (e) => {
      console.log("Message from server:", e.data);
      setServerMessage(e?.data); // Store the server message
    };

    ws.onerror = (e) => {
      console.log("WebSocket error:", e);
      setIsConnected(false); // Update state if there is an error
    };

    ws.onclose = (e) => {
      console.log("WebSocket connection closed:", e.code, e.reason);
      setIsConnected(false); // Update state if the connection closes
    };

    // Clean up WebSocket connection when the component unmounts
    return () => {
      ws.close();
    };
  }, []);

  return (
    <View>
      <Text style={{ color: "white" }}>
        {isConnected ? "Connected to WebSocket" : "Not connected to WebSocket"}
      </Text>
      {serverMessage ? (
        <Text style={{ color: "green" }}>Server: {serverMessage}</Text>
      ) : (
        <Text style={{ color: "gray" }}>No message from server yet</Text>
      )}
    </View>
  );
}