use leptos::prelude::*;

#[component]
pub fn DemoVercelInteraction() -> impl IntoView {
    view! {
        <div
            class="relative flex items-center justify-center w-full h-screen overflow-hidden"
            id="app"
        >
            <div class="z-10 flex flex-col items-center w-full">
                <div class="w-full border-b border-dashed border-slate-200"></div>
                <div class="flex flex-col items-center justify-center w-full h-full max-w-lg gap-8 p-16 mx-auto bg-white">
                    <div class="group isolate flex flex-col rounded-2xl bg-gray-900 shadow-[inset_0_1px,inset_0_0_0_1px] shadow-white/[0.025]">
                        <div class="relative z-10 flex-none order-last px-6 pb-6">
                            <h3 class="text-sm font-medium text-white">
                                Email and SMS one-time passcodes
                            </h3>
                            <p class="mt-2 text-gray-400 text-sm/5">
                                Fast and reliable one-time passcode delivery with built-in brute
                                force
                                prevention.
                            </p>
                        </div>
                        <div
                            class="relative flex-auto pointer-events-none select-none"
                            style="min-height:10.25rem"
                            aria-hidden="true"
                        >
                            <div
                                class="absolute inset-x-0 top-0 isolate h-[calc(206/16*1rem)] overflow-hidden pt-6 scale-[0.98] group-hover:scale-100 duration-300 ease-in-out group-hover:translate-y-[-2rem]"
                                style="will-change: auto;"
                            >
                                <div
                                    class="mx-auto h-56 w-[calc(264/16*1rem)] rounded-[calc(44/16*1rem)] bg-gray-800 p-1.5"
                                    style="box-shadow:0 1px 0 0 rgb(255 255 255 / 0.05) inset, 0px 2px 5px 0 rgb(0 0 0 / 0.40);background-image:linear-gradient(180deg, rgb(255 255 255 / 0.05) 0%, rgb(255 255 255 / 0) 67.19%)"
                                >
                                    <div class="relative h-[calc(200/16*1rem)] overflow-hidden rounded-[calc(38/16*1rem)] bg-gray-950/50 px-5 pt-3 ring-1 ring-inset ring-black/5">
                                        <div
                                            class="relative z-10 mx-auto flex h-6 w-6 transform-gpu items-center justify-center rounded-full group-hover:bg-[#5EE4FF] bg-[#131316] duration-300 ease"
                                            style="box-shadow: rgba(255, 255, 255, 0.05) 0px 1px; will-change: auto;"
                                        >
                                            <svg viewBox="0 0 16 16" class="w-4 h-4" aria-hidden="true">
                                                <path
                                                    fill="#fff"
                                                    fill-opacity=".4"
                                                    d="M3 9a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V9Z"
                                                    opacity="0"
                                                ></path>
                                                <path
                                                    fill="#fff"
                                                    fill-opacity=".4"
                                                    fill-rule="evenodd"
                                                    d="M8 4a2.5 2.5 0 0 0-2.5 2.5V10h-1V6.5a3.5 3.5 0 1 1 7 0V10h-1V6.5A2.5 2.5 0 0 0 8 4Z"
                                                    clip-rule="evenodd"
                                                    opacity="0"
                                                ></path>
                                                <path
                                                    d="M3 8a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8Z"
                                                    fill="rgba(116, 118, 134, 1)"
                                                ></path>
                                                <path
                                                    fill-rule="evenodd"
                                                    d="M8 3a2.5 2.5 0 0 0-2.5 2.5V9h-1V5.5a3.5 3.5 0 1 1 7 0V9h-1V5.5A2.5 2.5 0 0 0 8 3Z"
                                                    clip-rule="evenodd"
                                                    fill="rgba(116, 118, 134, 1)"
                                                ></path>
                                            </svg>
                                        </div>
                                        <span class="[perspective:1000px]">
                                            <div
                                                class="absolute inset-x-2 top-12 z-20 flex origin-top items-center gap-x-3 rounded-2xl bg-gray-800 p-2 translate-y-[-6.5rem] scale-90 opacity-50 group-hover:scale-100 group-hover:opacity-100 group-hover:translate-y-0 duration-300 ease-in-out group-hover:blur-0 blur-[2px] delay-150"
                                                style="box-shadow: rgba(19, 19, 22, 0.6) 0px 6px 12px, rgba(255, 255, 255, 0.03) 0px 1px inset; will-change: transform;"
                                            >
                                                <div
                                                    class="flex h-10 w-10 flex-none items-center justify-center rounded-[calc(10/16*1rem)] bg-gray-700"
                                                    style="box-shadow:0 1px rgb(255 255 255 / 0.05) inset;background-image:radial-gradient(circle at top, rgb(114 233 255 / 0.2), rgb(114 233 255 / 0))"
                                                >
                                                    <svg
                                                        viewBox="0 0 40 40"
                                                        fill="none"
                                                        aria-hidden="true"
                                                        class="size-10"
                                                    >
                                                        <g filter="url(#filter0_di_5116_3367)">
                                                            <path
                                                                fill="#5DE3FF"
                                                                fill-rule="evenodd"
                                                                d="M20 32c6.627 0 12-5.373 12-12S26.627 8 20 8 8 13.373 8 20s5.373 12 12 12Zm6-12c0 2.761-2.686 5-6 5a7.2 7.2 0 0 1-1.163-.094 1.227 1.227 0 0 0-.79.14c-.613.34-1.308.571-1.983.72-.82.182-1.314-.759-.895-1.485.04-.07.08-.14.119-.212.21-.382.099-.846-.184-1.178C14.409 22.075 14 21.077 14 20c0-2.761 2.686-5 6-5s6 2.239 6 5Z"
                                                                clip-rule="evenodd"
                                                            ></path>
                                                        </g>
                                                        <defs>
                                                            <filter
                                                                id="filter0_di_5116_3367"
                                                                width="42"
                                                                height="42"
                                                                x="-1"
                                                                y="-1"
                                                                color-interpolation-filters="sRGB"
                                                                filterUnits="userSpaceOnUse"
                                                            >
                                                                <feFlood
                                                                    flood-opacity="0"
                                                                    result="BackgroundImageFix"
                                                                ></feFlood>
                                                                <feColorMatrix
                                                                    in="SourceAlpha"
                                                                    result="hardAlpha"
                                                                    values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"
                                                                ></feColorMatrix>
                                                                <feMorphology
                                                                    in="SourceAlpha"
                                                                    operator="dilate"
                                                                    radius="1"
                                                                    result="effect1_dropShadow_5116_3367"
                                                                ></feMorphology>
                                                                <feOffset></feOffset>
                                                                <feGaussianBlur stdDeviation="4"></feGaussianBlur>
                                                                <feComposite in2="hardAlpha" operator="out"></feComposite>
                                                                <feColorMatrix values="0 0 0 0 0.419608 0 0 0 0 0.905882 0 0 0 0 1 0 0 0 0.3 0"></feColorMatrix>
                                                                <feBlend
                                                                    in2="BackgroundImageFix"
                                                                    result="effect1_dropShadow_5116_3367"
                                                                ></feBlend>
                                                                <feBlend
                                                                    in="SourceGraphic"
                                                                    in2="effect1_dropShadow_5116_3367"
                                                                    result="shape"
                                                                ></feBlend>
                                                                <feColorMatrix
                                                                    in="SourceAlpha"
                                                                    result="hardAlpha"
                                                                    values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"
                                                                ></feColorMatrix>
                                                                <feOffset dy="1"></feOffset>
                                                                <feComposite
                                                                    in2="hardAlpha"
                                                                    k2="-1"
                                                                    k3="1"
                                                                    operator="arithmetic"
                                                                ></feComposite>
                                                                <feColorMatrix values="0 0 0 0 1 0 0 0 0 1 0 0 0 0 1 0 0 0 0.2 0"></feColorMatrix>
                                                                <feBlend
                                                                    in2="shape"
                                                                    result="effect2_innerShadow_5116_3367"
                                                                ></feBlend>
                                                            </filter>
                                                        </defs>
                                                    </svg>
                                                </div>
                                                <div class="min-w-0">
                                                    <div class="text-[0.625rem]/4 font-medium text-[#5DE3FF]">
                                                        Security alert
                                                    </div>
                                                    <div class="text-gray-200 truncate text-xs/4">
                                                        Your security passcode is
                                                        <span class="text-white">764676</span>
                                                    </div>
                                                </div>
                                            </div>
                                        </span>
                                        <div class="flex flex-wrap justify-between mt-6 text-center gap-x-2 gap-y-4">
                                            <div class="flex-none">
                                                <div
                                                    class="relative size-10 rounded-[calc(10/16*1rem)] bg-gray-800"
                                                    style="box-shadow:0 1px rgb(255 255 255 / 0.05) inset"
                                                >
                                                    <svg
                                                        viewBox="0 0 40 40"
                                                        fill="none"
                                                        aria-hidden="true"
                                                        class="size-10"
                                                    >
                                                        <g filter="url(#email-sms-icon-shadow)">
                                                            <path
                                                                fill="#5E5F6E"
                                                                d="m31.661 28.642-1.576 2.465c-.126.197-.266.388-.465.509-1.796 1.084-7.63.18-14.523-6.713-7.302-7.303-7.883-13.416-6.5-14.799l2.761-1.765a2.152 2.152 0 0 1 2.68.291l2.119 2.118c.714.714.835 1.83.29 2.68l-1.18 1.847c-.387.607-.657 1.29-.451 1.98.373 1.253 1.406 3.24 3.047 4.882a12.282 12.282 0 0 0 3.656 2.515c1.425.633 2.997.214 4.311-.626l.742-.474a2.151 2.151 0 0 1 2.68.291l2.118 2.119c.714.714.835 1.829.291 2.68Z"
                                                            ></path>
                                                            <path
                                                                fill="url(#paint0_linear_5116_3351)"
                                                                fill-opacity=".2"
                                                                d="m31.661 28.642-1.576 2.465c-.126.197-.266.388-.465.509-1.796 1.084-7.63.18-14.523-6.713-7.302-7.303-7.883-13.416-6.5-14.799l2.761-1.765a2.152 2.152 0 0 1 2.68.291l2.119 2.118c.714.714.835 1.83.29 2.68l-1.18 1.847c-.387.607-.657 1.29-.451 1.98.373 1.253 1.406 3.24 3.047 4.882a12.282 12.282 0 0 0 3.656 2.515c1.425.633 2.997.214 4.311-.626l.742-.474a2.151 2.151 0 0 1 2.68.291l2.118 2.119c.714.714.835 1.829.291 2.68Z"
                                                            ></path>
                                                        </g>
                                                        <defs>
                                                            <linearGradient
                                                                id="paint0_linear_5116_3351"
                                                                x1="20"
                                                                x2="20"
                                                                y1="9"
                                                                y2="31"
                                                                gradientUnits="userSpaceOnUse"
                                                            >
                                                                <stop stop-color="#fff"></stop>
                                                                <stop offset="1" stop-opacity="0"></stop>
                                                            </linearGradient>
                                                            <filter
                                                                id="email-sms-icon-shadow"
                                                                width="30"
                                                                height="30"
                                                                x="5"
                                                                y="6"
                                                                color-interpolation-filters="sRGB"
                                                                filterUnits="userSpaceOnUse"
                                                            >
                                                                <feFlood
                                                                    flood-opacity="0"
                                                                    result="BackgroundImageFix"
                                                                ></feFlood>
                                                                <feColorMatrix
                                                                    in="SourceAlpha"
                                                                    result="hardAlpha"
                                                                    values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"
                                                                ></feColorMatrix>
                                                                <feOffset dy="1"></feOffset>
                                                                <feGaussianBlur stdDeviation="1.5"></feGaussianBlur>
                                                                <feComposite in2="hardAlpha" operator="out"></feComposite>
                                                                <feColorMatrix values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0.2 0"></feColorMatrix>
                                                                <feBlend
                                                                    in2="BackgroundImageFix"
                                                                    result="effect1_dropShadow_5116_3351"
                                                                ></feBlend>
                                                                <feBlend
                                                                    in="SourceGraphic"
                                                                    in2="effect1_dropShadow_5116_3351"
                                                                    result="shape"
                                                                ></feBlend>
                                                                <feColorMatrix
                                                                    in="SourceAlpha"
                                                                    result="hardAlpha"
                                                                    values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"
                                                                ></feColorMatrix>
                                                                <feOffset dy="1"></feOffset>
                                                                <feComposite
                                                                    in2="hardAlpha"
                                                                    k2="-1"
                                                                    k3="1"
                                                                    operator="arithmetic"
                                                                ></feComposite>
                                                                <feColorMatrix values="0 0 0 0 1 0 0 0 0 1 0 0 0 0 1 0 0 0 0.08 0"></feColorMatrix>
                                                                <feBlend
                                                                    in2="shape"
                                                                    result="effect2_innerShadow_5116_3351"
                                                                ></feBlend>
                                                            </filter>
                                                        </defs>
                                                    </svg>
                                                </div>
                                                <div class="mt-1.5 text-[0.625rem]/4 font-medium text-gray-300">
                                                    Phone
                                                </div>
                                            </div>
                                            <div class="flex-none">
                                                <div
                                                    class="relative size-10 rounded-[calc(10/16*1rem)] bg-gray-800"
                                                    style="box-shadow:0 1px rgb(255 255 255 / 0.05) inset"
                                                >
                                                    <svg
                                                        viewBox="0 0 40 40"
                                                        fill="none"
                                                        aria-hidden="true"
                                                        class="size-10"
                                                    >
                                                        <g
                                                            fill-rule="evenodd"
                                                            clip-rule="evenodd"
                                                            filter="url(#email-sms-icon-shadow)"
                                                        >
                                                            <path
                                                                fill="#5E5F6E"
                                                                d="M20 32c6.627 0 12-5.373 12-12S26.627 8 20 8 8 13.373 8 20s5.373 12 12 12Zm6-12c0 2.761-2.686 5-6 5a7.2 7.2 0 0 1-1.163-.094 1.227 1.227 0 0 0-.79.14c-.613.34-1.308.571-1.983.72-.82.182-1.314-.759-.895-1.485.04-.07.08-.14.119-.212.21-.382.099-.846-.184-1.178C14.409 22.075 14 21.077 14 20c0-2.761 2.686-5 6-5s6 2.239 6 5Z"
                                                            ></path>
                                                            <path
                                                                fill="url(#paint0_linear_5116_3354)"
                                                                fill-opacity=".2"
                                                                d="M20 32c6.627 0 12-5.373 12-12S26.627 8 20 8 8 13.373 8 20s5.373 12 12 12Zm6-12c0 2.761-2.686 5-6 5a7.2 7.2 0 0 1-1.163-.094 1.227 1.227 0 0 0-.79.14c-.613.34-1.308.571-1.983.72-.82.182-1.314-.759-.895-1.485.04-.07.08-.14.119-.212.21-.382.099-.846-.184-1.178C14.409 22.075 14 21.077 14 20c0-2.761 2.686-5 6-5s6 2.239 6 5Z"
                                                            ></path>
                                                        </g>
                                                        <defs>
                                                            <linearGradient
                                                                id="paint0_linear_5116_3354"
                                                                x1="20"
                                                                x2="20"
                                                                y1="9"
                                                                y2="31"
                                                                gradientUnits="userSpaceOnUse"
                                                            >
                                                                <stop stop-color="#fff"></stop>
                                                                <stop offset="1" stop-opacity="0"></stop>
                                                            </linearGradient>
                                                        </defs>
                                                    </svg>
                                                    <div
                                                        class="absolute -left-1.5 -top-1.5 z-10 flex h-5 w-5 items-center justify-center rounded-full text-[0.625rem]/none font-semibold text-gray-950 backdrop-blur scale-75 group-hover:scale-100 duration-100 ease bg-[#ffffff40] group-hover:bg-[#5EE4FF]"
                                                        style="will-change: auto; box-shadow: rgba(107, 231, 255, 0.3) 0px 0px 0px 0px, rgba(255, 255, 255, 0.2) 0px 0px inset;"
                                                    >
                                                        1
                                                    </div>
                                                </div>
                                                <div class="mt-1.5 text-[0.625rem]/4 font-medium text-gray-300">
                                                    SMS
                                                </div>
                                            </div>
                                            <div class="flex-none">
                                                <div
                                                    class="relative size-10 rounded-[calc(10/16*1rem)] bg-gray-800"
                                                    style="box-shadow:0 1px rgb(255 255 255 / 0.05) inset"
                                                >
                                                    <svg
                                                        viewBox="0 0 40 40"
                                                        fill="none"
                                                        aria-hidden="true"
                                                        class="size-10"
                                                    >
                                                        <g filter="url(#email-sms-icon-shadow)">
                                                            <path
                                                                fill="#5E5F6E"
                                                                d="M19.083 13.241a.768.768 0 0 0-.179-.523c-.594-.657-2.828-2.545-8.984-2.718-.506-.014-.92.429-.92.973v16.746c0 .544.414.984.92.998 3.126.088 5.241.617 6.643 1.204.846.354 2.52-.54 2.52-1.515V13.24Z"
                                                            ></path>
                                                            <path
                                                                fill="url(#paint0_linear_5116_3361)"
                                                                fill-opacity=".2"
                                                                d="M19.083 13.241a.768.768 0 0 0-.179-.523c-.594-.657-2.828-2.545-8.984-2.718-.506-.014-.92.429-.92.973v16.746c0 .544.414.984.92.998 3.126.088 5.241.617 6.643 1.204.846.354 2.52-.54 2.52-1.515V13.24Z"
                                                            ></path>
                                                            <path
                                                                fill="#5E5F6E"
                                                                d="M20.917 13.241c0-.194.053-.383.179-.523.594-.657 2.828-2.545 8.984-2.718.506-.014.92.429.92.973v16.746c0 .544-.414.984-.92.998-3.126.088-5.241.617-6.643 1.204-.846.354-2.52-.54-2.52-1.515V13.24Z"
                                                            ></path>
                                                            <path
                                                                fill="url(#paint1_linear_5116_3361)"
                                                                fill-opacity=".2"
                                                                d="M20.917 13.241c0-.194.053-.383.179-.523.594-.657 2.828-2.545 8.984-2.718.506-.014.92.429.92.973v16.746c0 .544-.414.984-.92.998-3.126.088-5.241.617-6.643 1.204-.846.354-2.52-.54-2.52-1.515V13.24Z"
                                                            ></path>
                                                        </g>
                                                        <defs>
                                                            <linearGradient
                                                                id="paint0_linear_5116_3361"
                                                                x1="20"
                                                                x2="20"
                                                                y1="10"
                                                                y2="30"
                                                                gradientUnits="userSpaceOnUse"
                                                            >
                                                                <stop stop-color="#fff"></stop>
                                                                <stop offset="1" stop-opacity="0"></stop>
                                                            </linearGradient>
                                                            <linearGradient
                                                                id="paint1_linear_5116_3361"
                                                                x1="20"
                                                                x2="20"
                                                                y1="10"
                                                                y2="30"
                                                                gradientUnits="userSpaceOnUse"
                                                            >
                                                                <stop stop-color="#fff"></stop>
                                                                <stop offset="1" stop-opacity="0"></stop>
                                                            </linearGradient>
                                                        </defs>
                                                    </svg>
                                                </div>
                                                <div class="mt-1.5 text-[0.625rem]/4 font-medium text-gray-300">
                                                    Books
                                                </div>
                                            </div>
                                            <div class="flex-none">
                                                <div
                                                    class="relative size-10 rounded-[calc(10/16*1rem)] bg-gray-800"
                                                    style="box-shadow:0 1px rgb(255 255 255 / 0.05) inset"
                                                >
                                                    <svg
                                                        viewBox="0 0 40 40"
                                                        fill="none"
                                                        aria-hidden="true"
                                                        class="size-10"
                                                    >
                                                        <g filter="url(#email-sms-icon-shadow)">
                                                            <path
                                                                fill="#5E5F6E"
                                                                fill-rule="evenodd"
                                                                d="M14 30a1 1 0 0 1 1-1h9a1 1 0 1 1 0 2h-9a1 1 0 0 1-1-1Z"
                                                                clip-rule="evenodd"
                                                            ></path>
                                                            <path
                                                                fill="url(#paint0_linear_5116_3364)"
                                                                fill-opacity=".2"
                                                                fill-rule="evenodd"
                                                                d="M14 30a1 1 0 0 1 1-1h9a1 1 0 1 1 0 2h-9a1 1 0 0 1-1-1Z"
                                                                clip-rule="evenodd"
                                                            ></path>
                                                            <path
                                                                fill="#5E5F6E"
                                                                d="M8 13a2 2 0 0 1 2-2h20a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H10a2 2 0 0 1-2-2V13Z"
                                                            ></path>
                                                            <path
                                                                fill="url(#paint1_linear_5116_3364)"
                                                                fill-opacity=".2"
                                                                d="M8 13a2 2 0 0 1 2-2h20a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H10a2 2 0 0 1-2-2V13Z"
                                                            ></path>
                                                        </g>
                                                        <defs>
                                                            <linearGradient
                                                                id="paint0_linear_5116_3364"
                                                                x1="20"
                                                                x2="20"
                                                                y1="11"
                                                                y2="31"
                                                                gradientUnits="userSpaceOnUse"
                                                            >
                                                                <stop stop-color="#fff"></stop>
                                                                <stop offset="1" stop-opacity="0"></stop>
                                                            </linearGradient>
                                                            <linearGradient
                                                                id="paint1_linear_5116_3364"
                                                                x1="20"
                                                                x2="20"
                                                                y1="11"
                                                                y2="31"
                                                                gradientUnits="userSpaceOnUse"
                                                            >
                                                                <stop stop-color="#fff"></stop>
                                                                <stop offset="1" stop-opacity="0"></stop>
                                                            </linearGradient>
                                                        </defs>
                                                    </svg>
                                                </div>
                                                <div class="mt-1.5 text-[0.625rem]/4 font-medium text-gray-300">
                                                    TV
                                                </div>
                                            </div>
                                            <div class="flex-none">
                                                <div
                                                    class="relative size-10 rounded-[calc(10/16*1rem)] bg-gray-800"
                                                    style="box-shadow:0 1px rgb(255 255 255 / 0.05) inset"
                                                ></div>
                                                <div class="mt-1.5 text-[0.625rem]/4 font-medium text-gray-300">
                                                    Google
                                                </div>
                                            </div>
                                            <div class="flex-none">
                                                <div
                                                    class="relative size-10 rounded-[calc(10/16*1rem)] bg-gray-800"
                                                    style="box-shadow:0 1px rgb(255 255 255 / 0.05) inset"
                                                ></div>
                                                <div class="mt-1.5 text-[0.625rem]/4 font-medium text-gray-300">
                                                    X
                                                </div>
                                            </div>
                                            <div class="flex-none">
                                                <div
                                                    class="relative size-10 rounded-[calc(10/16*1rem)] bg-gray-800"
                                                    style="box-shadow:0 1px rgb(255 255 255 / 0.05) inset"
                                                ></div>
                                                <div class="mt-1.5 text-[0.625rem]/4 font-medium text-gray-300">
                                                    NatWest
                                                </div>
                                            </div>
                                            <div class="flex-none">
                                                <div
                                                    class="relative size-10 rounded-[calc(10/16*1rem)] bg-gray-800"
                                                    style="box-shadow:0 1px rgb(255 255 255 / 0.05) inset"
                                                ></div>
                                                <div class="mt-1.5 text-[0.625rem]/4 font-medium text-gray-300">
                                                    Clerk
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                <div
                                    class="absolute inset-0 bg-gradient-to-t from-gray-900 group-hover:translate-y-[2rem] duration-300 ease-in-out"
                                    style="will-change: auto;"
                                ></div>
                            </div>
                        </div>
                    </div>

                </div>
                <div class="w-full border-t border-dashed border-slate-200"></div>
            </div>
            <div class="absolute inset-0 w-full h-full">
                <div class="absolute inset-0 w-full h-full px-4 py-0 pointer-events-none">
                    <div class="flex justify-between items-center h-full max-w-[1080px] mx-auto">
                        <div class="bg-slate-200 w-[1px] h-full"></div>
                        <div class="h-full border-r border-dashed border-slate-200"></div>
                        <div class="h-full border-r border-dashed border-slate-200"></div>
                        <div class="h-full border-r border-dashed border-slate-200"></div>
                        <div class="bg-slate-200 w-[1px] h-full"></div>
                    </div>
                </div>
            </div>

        </div>
    }
}
