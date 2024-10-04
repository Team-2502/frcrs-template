'use client';

import React, {useEffect, useState} from "react";
import {Button, Select, SelectItem} from "@nextui-org/react";
import {wait} from "next/dist/lib/wait";

async function get(endpoint: string): Promise<string> {
    const res = await fetch('/' + endpoint);
    return await res.text()
}

async function put(endpoint: string, data: string): Promise<void> {
    const xhr = new XMLHttpRequest()
    xhr.open("PUT", '/' + endpoint, true)
    xhr.setRequestHeader('Content-Type', 'application/json')
    xhr.send(data)
}

export default function Home() {
    const [autos, setAutos] = useState<string[] | null>(null)
    const [selected, setSelected] = useState(0)
    const [hz, setHz] = useState(0)

    useEffect(() => {
        setInterval(() => get("telemetry/auto chooser").then(value => setAutos(JSON.parse(value))), 750)
        setInterval(() => get("telemetry/selected auto").then(value => setSelected(Number.parseFloat(value))), 750)
        setInterval(() => get("telemetry/loop rate (hz)").then(value => setHz(Number.parseFloat(JSON.parse(value)))), 500)
    }, []);

  if (!autos) return (<></>);

  return (
      <section className="flex flex-col items-center justify-center gap-4 py-8 md:py-10">
          <a>{"Hz: " + hz.toFixed(2)}</a>
          <div className="flex flex-col gap-4 w-full">

              {autos.map((auto, idx) => {
                  return (
                      <Button className="w-72" key={idx} onPress={() => put("telemetry/selected auto", idx.toString())} variant="solid" style={(selected == idx) ? {background:  "green"}: {}}>{auto}</Button>
                  )
              })}
          </div>
      </section>
  );
}
