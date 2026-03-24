import Image from "next/image";

export default function Home() {
  return (
    <div className="flex min-h-screen flex-col bg-white">
      <main className="flex1">
        {/* hero section */}
        <section className="container mx-auto px-4 py-32">
          <div className="mx-auto max-w-4xl text-center">
            <h1 className="text-black mb-6 text-6xl font-bold">
              A better way to track your job application
            </h1>
            <p className="text-gray-700 text-mutex-foregroud mb-10 text-xl">
              Capture, organise and manage your job search in one place.
            </p>
            <div className="flex flex-col items-center gap-4">
              <button>Start for free</button>
              <p>Free forever, no credit card required.</p>
            </div>
          </div>
        </section>
      </main>
    </div>
  );
}
