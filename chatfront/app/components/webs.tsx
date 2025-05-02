import { View, Text , Button, TextInput} from "react-native";
import React, { useEffect, useState } from "react";
import EventSource from "react-native-sse";

export default function Webs() {
  const [isConnected, setIsConnected] = useState<boolean>(false);
  const [serverMessage, setServerMessage] = useState("");
  const [ws, setWS] = useState<WebSocket | null>(null);
  const [inputMessage, setInputMessage] = useState<string>(""); // State for input text

  //const es = new EventSource("http://localhost:8000/events");
  let url = "http://localhost/api/events";
  const es = new EventSource(url, {
    headers: {
      "Content-Type": "application/json",
      //Authorization: `Bearer ${OPENAI_KEY}`,
    },
   // method: "GET",
    //body: JSON.stringify(data),
    pollingInterval: 3000,
  });


  es.addEventListener("open", (event) => {
    console.log("Open SSE connection.");

  });
  
  es.addEventListener("message", (event) => {
    console.log("New message event:", event.data);
    setServerMessage(event.data.toString());
  });
  
  es.addEventListener("error", (event) => {
    if (event.type === "error") {
      console.error("Connection error:", event.message);
    } else if (event.type === "exception") {
      console.error("Error:", event.message, event.error);
    }
  });
  
  es.addEventListener("close", (event) => {
    console.log("Close SSE connection.");
  });

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
       <TextInput
        style={{
          height: 40,
          borderColor: "gray",
          borderWidth: 1,
          marginVertical: 10,
          paddingHorizontal: 8,
          color: "blue",
        }}
        placeholder="Enter your message"
        placeholderTextColor="gray"
        value={inputMessage}
        onChangeText={setInputMessage}
      />

    </View>
  );
}