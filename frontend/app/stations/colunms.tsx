import { ColumnDef } from "@tanstack/react-table";
import Link from "next/link";

export type Station = {
    id: string,
    name_fi: string
}

export const columns: ColumnDef<Station>[] = [
    /*
    {
        accessorKey: "id",
        header: "ID"
    },
    */
    {
        accessorKey: "name_fi",
        header: "Name",
        cell: ({row}: any) => {
            const station: Station = row.original
            const id = station.id.substring(10)
            return (
                <Link href={`/stations/${id}`}>{station.name_fi}</Link>
                )
        }
    },
]