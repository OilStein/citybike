import { ColumnDef } from "@tanstack/react-table";
import Link from "next/link";

export type Journey = {
    id: string,
    dep_station_id: number,
    dep_station_name: string,
    tar_station_id: number,
    tar_station_name: string,
    distance: number,
    duration: number,
}

// converts meters to kilometers
export const kilometer = (m: number):number => {
    return +(m / 1000).toFixed(1);
  }

  // converts seconds to minutes, rounding up at trailing 30
export const minutes = (sec: number): number => {
    if(sec < 30) return 0
    let mins = Math.floor(sec/60)
    let trailingSecs = sec - (mins * 60)
    if (trailingSecs >= 30) return mins + 1
    if (mins <= 0) return 1;
    return mins
  }

export const columns: ColumnDef<Journey>[] = [
    {
        accessorKey: "dep_station_name",
        header: "Departure",
        cell: ({row}:any) => {
            const journey: Journey = row.original
            return (
                <Link href={`/stations/${journey.dep_station_id}`}>{journey.dep_station_name}</Link>
                )
        }
    },
    {
        accessorKey: "tar_station_name",
        header: "Return",
        cell: ({row}:any) => {
            const journey: Journey = row.original
            return (
                <Link href={`/stations/${journey.tar_station_id}`}>{journey.tar_station_name}</Link>
                )
        }
    },
    {
        accessorKey: "distance",
        header: "Distance",
        cell: ({row}:any) => {
            const journey: Journey = row.original
            const formatted = kilometer(journey.distance)

            return <div>{formatted} km</div>
        }
    },
    {
        accessorKey: "duration",
        header: "Duration",
        cell: ({row}:any) => {
            const journey: Journey = row.original
            const formatted = minutes(journey.duration)

            return <div>{formatted} min</div>
        }
    }
]
