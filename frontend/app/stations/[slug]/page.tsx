"use client"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import NavBar from "@/components/ui/navbar"
import useSWR, { Fetcher } from "swr"


const fetcher: Fetcher<Station, string> = (url: string) => fetch(url).then((res) => res.json())

type Station = {
    id: string,
    name_fi: string,
    address_fi: string,
    capacity: number,
    latitude: number,
    longitude: number,
    data: {
        starting: number,
        ending: number
    }
}


const SingleStation = ({params}: {params: {slug: number}}) => {
    const {data, error} = useSWR(`http://localhost:8080/stations/${params.slug}`, fetcher)

    if (error) return <div>Error finding station</div>
    // console.log(data)
    return (
        <div className="container mx-auto">
            <NavBar></NavBar>

        <div className="container mx-auto max-w-[400px]">
            <Card className="">
                <CardHeader>
                    <CardTitle>{data?.name_fi}</CardTitle>
                    <CardDescription>{data?.address_fi}</CardDescription>
                </CardHeader>
                <CardContent>
                    <ul>
                        <li className="text-sm">Starting journeys: {data?.data.starting}</li>
                        <li className="text-sm">Ending journeys: {data?.data.ending}</li>
                    </ul>
                </CardContent>
            </Card>
        </div>
        </div>
    )
}

export default SingleStation