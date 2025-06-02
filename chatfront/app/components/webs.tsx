import { View, Text , Button, TextInput} from "react-native";
import React, { useEffect, useState } from "react";
import EventSource from "react-native-sse";

const Message = {
  room: "", // String with a maximum length of 30 characters
  username: "", // String with a maximum length of 20 characters
  message: "" // String
};

export default function Webs() {
  const [isConnected, setIsConnected] = useState<boolean>(false);
  const [serverMessage, setServerMessage] = useState<Array<{ room: string; username: string; message: string }>>([]);
  const [ws, setWS] = useState<WebSocket | null>(null);
  const [inputMessage, setInputMessage] = useState<string>(""); // State for input text

  //const es = new EventSource("http://localhost:8000/events");
  useEffect(() => {
  const  url = "/api/events";
  const es = new EventSource(url, {
    headers: {
      "Content-Type": "application/json",
      //Authorization: `Bearer ${OPENAI_KEY}`,
    },
   // method: "GET",
    //body: JSON.stringify(data),
    pollingInterval: 30000,
  });


  es.addEventListener("open", (event) => {
    setIsConnected(true);
    console.log("Open SSE connection.");
  });
  
  es.addEventListener("message", (e) => {
    console.log("Add SSE message listner");
    try {
      const jsonMessage = JSON.parse(e.data); // Parse incoming message as JSON
      console.error(" Message pre-transofrm", jsonMessage);
      const newMessage = {
        room: jsonMessage.room || "",
        username: jsonMessage.username || "",
        message: jsonMessage.message || "",
      }; // Ensure it matches the Message structure
     
      setServerMessage((prevMessages) => [...prevMessages, newMessage]); // Add to the list
    } catch (error) {
      console.error("Failed to parse message as JSON:", e.data);
    }
  });
  
  es.addEventListener("error", (event) => {
    if (event.type === "error") {
      console.error("Connection error:", event.message);
    } else if (event.type === "exception") {
      console.error("Error:", event.message, event.error);
    }
    else {
      console.log("Error");
    }
  });
  
  es.addEventListener("close", (event) => {
    setIsConnected(false);
    console.log("Close SSE connection.");
  });
  
   // Cleanup function to close the EventSource and remove listeners
   return () => {
    es.close();
    console.log("EventSource closed and listeners removed.");
  };
}, []); // Empty dependency array ensures this runs only once

  return (
    <View  style={{ padding: 20 }}>
      <Text style={{ color: "red" }}>
        {isConnected ? "Connected to WebSocket" : "Not connected to WebSocket"}
      </Text>
      {serverMessage.length > 0 ? (
      <View>
        {serverMessage.map((msg, index) => (
          <Text key={index} style={{ color: "green", marginVertical: 5 }}>
            Room: {msg.room}, User: {msg.username}, Message: {msg.message}
          </Text>
        ))}
      </View>
    ) : (
      <Text style={{ color: "gray" }}>No message from server yet</Text>
    )}


    </View>
  );
}