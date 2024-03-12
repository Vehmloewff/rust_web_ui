use rust_web_ui::*;

pub struct Example34;

pub struct Example34Props {}

impl Default for Example34Props {
	fn default() -> Example34Props {
		Example34Props { }
	}
}

impl Widget<'_> for Example34 {
	type Props = Example34Props;

	fn render(mut ctx: Ctx<'_>, props: Example34Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[NoStyle::Noop("isolate"), NoStyle::Noop("bg-white"), Style::PaddingX(24), Style::PaddingY(96), Screen::Small(&[Style::PaddingY(128)]), Screen::Large(&[Style::PaddingX(32)])]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-x-0"), NoStyle::Noop("top-[-10rem]"), NoStyle::Noop("-z-10"), NoStyle::Noop("transform-gpu"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("blur-3xl"), Screen::Small(&[NoStyle::Noop("top-[-20rem]")])]);
				props.set_attribute("aria-hidden", "true");
				
				props.child("1", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("left-1/2"), NoStyle::Noop("-z-10"), NoStyle::Noop("aspect-[1155/678]"), NoStyle::Noop("w-[36.125rem]"), NoStyle::Noop("max-w-none"), NoStyle::Noop("-translate-x-1/2"), NoStyle::Noop("rotate-[30deg]"), NoStyle::Noop("bg-gradient-to-tr"), NoStyle::Noop("from-[#ff80b5]"), NoStyle::Noop("to-[#9089fc]"), NoStyle::Noop("opacity-30"), Screen::Small(&[NoStyle::Noop("left-[calc(50%-40rem)]")]), Screen::Small(&[NoStyle::Noop("w-[72.1875rem]")])]);
					props.set_attribute("style", "clip-path: polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)");
				});
			});
			props.child("3", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-2xl"), NoStyle::Noop("text-center")]);
				
				props.child("1", Dynamic).run("h2", |props| {
					props.styles(&[NoStyle::Noop("text-3xl"), Style::FontBold, NoStyle::Noop("tracking-tight"), Style::TextColor(Color::Fg(100)), Screen::Small(&[NoStyle::Noop("text-4xl")])]);
					
					props.child("0", Label).run(|props| props.set_text("Contact sales"));
				});
				props.child("3", Dynamic).run("p", |props| {
					props.styles(&[Style::MarginTop(8), NoStyle::Noop("text-lg"), NoStyle::Noop("leading-8"), Style::TextColor(Color::Fg(67))]);
					
					props.child("0", Label).run(|props| props.set_text("Aute magna irure deserunt veniam aliqua magna enim voluptate."));
				});
			});
			props.child("5", Dynamic).run("form", |props| {
				props.set_attribute("action", "#");
				props.set_attribute("method", "POST");
				props.styles(&[NoStyle::Noop("mx-auto"), Style::MarginTop(64), NoStyle::Noop("max-w-xl"), Screen::Small(&[Style::MarginTop(80)])]);
				
				props.child("1", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("grid"), NoStyle::Noop("grid-cols-1"), NoStyle::Noop("gap-x-8"), NoStyle::Noop("gap-y-6"), Screen::Small(&[NoStyle::Noop("grid-cols-2")])]);
					
					props.child("1", Dynamic).run("div", |props| {
						props.child("1", Dynamic).run("label", |props| {
							props.set_attribute("for", "first-name");
							props.styles(&[Style::Block, NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.set_text("First name"));
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[Style::MarginTop(10)]);
							
							props.child("1", Dynamic).run("input", |props| {
								props.set_attribute("type", "text");
								props.set_attribute("name", "first-name");
								props.set_attribute("id", "first-name");
								props.set_attribute("autocomplete", "given-name");
								props.styles(&[Style::Block, NoStyle::Noop("w-full"), NoStyle::Noop("rounded-md"), NoStyle::Noop("border-0"), Style::PaddingX(14), Style::PaddingY(8), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), NoStyle::NoopGroup("placeholder", Style::TextColor(Color::Fg(44))), Action::Hover(&[NoStyle::Noop("ring-2")]), Action::Hover(&[NoStyle::Noop("ring-inset")]), Action::Hover(&[NoStyle::Noop("ring-indigo-600")]), Screen::Small(&[NoStyle::Noop("text-sm")]), Screen::Small(&[NoStyle::Noop("leading-6")])]);
							});
						});
					});
					props.child("3", Dynamic).run("div", |props| {
						props.child("1", Dynamic).run("label", |props| {
							props.set_attribute("for", "last-name");
							props.styles(&[Style::Block, NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.set_text("Last name"));
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[Style::MarginTop(10)]);
							
							props.child("1", Dynamic).run("input", |props| {
								props.set_attribute("type", "text");
								props.set_attribute("name", "last-name");
								props.set_attribute("id", "last-name");
								props.set_attribute("autocomplete", "family-name");
								props.styles(&[Style::Block, NoStyle::Noop("w-full"), NoStyle::Noop("rounded-md"), NoStyle::Noop("border-0"), Style::PaddingX(14), Style::PaddingY(8), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), NoStyle::NoopGroup("placeholder", Style::TextColor(Color::Fg(44))), Action::Hover(&[NoStyle::Noop("ring-2")]), Action::Hover(&[NoStyle::Noop("ring-inset")]), Action::Hover(&[NoStyle::Noop("ring-indigo-600")]), Screen::Small(&[NoStyle::Noop("text-sm")]), Screen::Small(&[NoStyle::Noop("leading-6")])]);
							});
						});
					});
					props.child("5", Dynamic).run("div", |props| {
						props.styles(&[Screen::Small(&[NoStyle::Noop("col-span-2")])]);
						
						props.child("1", Dynamic).run("label", |props| {
							props.set_attribute("for", "company");
							props.styles(&[Style::Block, NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.set_text("Company"));
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[Style::MarginTop(10)]);
							
							props.child("1", Dynamic).run("input", |props| {
								props.set_attribute("type", "text");
								props.set_attribute("name", "company");
								props.set_attribute("id", "company");
								props.set_attribute("autocomplete", "organization");
								props.styles(&[Style::Block, NoStyle::Noop("w-full"), NoStyle::Noop("rounded-md"), NoStyle::Noop("border-0"), Style::PaddingX(14), Style::PaddingY(8), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), NoStyle::NoopGroup("placeholder", Style::TextColor(Color::Fg(44))), Action::Hover(&[NoStyle::Noop("ring-2")]), Action::Hover(&[NoStyle::Noop("ring-inset")]), Action::Hover(&[NoStyle::Noop("ring-indigo-600")]), Screen::Small(&[NoStyle::Noop("text-sm")]), Screen::Small(&[NoStyle::Noop("leading-6")])]);
							});
						});
					});
					props.child("7", Dynamic).run("div", |props| {
						props.styles(&[Screen::Small(&[NoStyle::Noop("col-span-2")])]);
						
						props.child("1", Dynamic).run("label", |props| {
							props.set_attribute("for", "email");
							props.styles(&[Style::Block, NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.set_text("Email"));
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[Style::MarginTop(10)]);
							
							props.child("1", Dynamic).run("input", |props| {
								props.set_attribute("type", "email");
								props.set_attribute("name", "email");
								props.set_attribute("id", "email");
								props.set_attribute("autocomplete", "email");
								props.styles(&[Style::Block, NoStyle::Noop("w-full"), NoStyle::Noop("rounded-md"), NoStyle::Noop("border-0"), Style::PaddingX(14), Style::PaddingY(8), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), NoStyle::NoopGroup("placeholder", Style::TextColor(Color::Fg(44))), Action::Hover(&[NoStyle::Noop("ring-2")]), Action::Hover(&[NoStyle::Noop("ring-inset")]), Action::Hover(&[NoStyle::Noop("ring-indigo-600")]), Screen::Small(&[NoStyle::Noop("text-sm")]), Screen::Small(&[NoStyle::Noop("leading-6")])]);
							});
						});
					});
					props.child("9", Dynamic).run("div", |props| {
						props.styles(&[Screen::Small(&[NoStyle::Noop("col-span-2")])]);
						
						props.child("1", Dynamic).run("label", |props| {
							props.set_attribute("for", "phone-number");
							props.styles(&[Style::Block, NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.set_text("Phone number"));
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("relative"), Style::MarginTop(10)]);
							
							props.child("1", Dynamic).run("div", |props| {
								props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-y-0"), NoStyle::Noop("left-0"), Style::Flex, Style::ItemsCenter]);
								
								props.child("1", Dynamic).run("label", |props| {
									props.set_attribute("for", "country");
									props.styles(&[NoStyle::Noop("sr-only")]);
									
									props.child("0", Label).run(|props| props.set_text("Country"));
								});
								props.child("3", Dynamic).run("select", |props| {
									props.set_attribute("id", "country");
									props.set_attribute("name", "country");
									props.styles(&[NoStyle::Noop("h-full"), NoStyle::Noop("rounded-md"), NoStyle::Noop("border-0"), NoStyle::Noop("bg-transparent"), NoStyle::Noop("bg-none"), Style::PaddingY(0), Style::PaddingLeft(16), Style::PaddingRight(36), Style::TextColor(Color::Fg(44)), Action::Hover(&[NoStyle::Noop("ring-2")]), Action::Hover(&[NoStyle::Noop("ring-inset")]), Action::Hover(&[NoStyle::Noop("ring-indigo-600")]), Screen::Small(&[NoStyle::Noop("text-sm")])]);
									
									props.child("1", Dynamic).run("option", |props| {
										props.child("0", Label).run(|props| props.set_text("US"));
									});
									props.child("3", Dynamic).run("option", |props| {
										props.child("0", Label).run(|props| props.set_text("CA"));
									});
									props.child("5", Dynamic).run("option", |props| {
										props.child("0", Label).run(|props| props.set_text("EU"));
									});
								});
								props.child("5", Icon).run(|props| {
									props.style(&[NoStyle::Noop("pointer-events-none"), NoStyle::Noop("absolute"), NoStyle::Noop("right-3"), NoStyle::Noop("top-0"), NoStyle::Noop("h-full"), Style::Width(20), Style::TextColor(Color::Fg(44))]);
								});
							});
							props.child("3", Dynamic).run("input", |props| {
								props.set_attribute("type", "tel");
								props.set_attribute("name", "phone-number");
								props.set_attribute("id", "phone-number");
								props.set_attribute("autocomplete", "tel");
								props.styles(&[Style::Block, NoStyle::Noop("w-full"), NoStyle::Noop("rounded-md"), NoStyle::Noop("border-0"), Style::PaddingX(14), Style::PaddingY(8), Style::PaddingLeft(80), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), NoStyle::NoopGroup("placeholder", Style::TextColor(Color::Fg(44))), Action::Hover(&[NoStyle::Noop("ring-2")]), Action::Hover(&[NoStyle::Noop("ring-inset")]), Action::Hover(&[NoStyle::Noop("ring-indigo-600")]), Screen::Small(&[NoStyle::Noop("text-sm")]), Screen::Small(&[NoStyle::Noop("leading-6")])]);
							});
						});
					});
					props.child("11", Dynamic).run("div", |props| {
						props.styles(&[Screen::Small(&[NoStyle::Noop("col-span-2")])]);
						
						props.child("1", Dynamic).run("label", |props| {
							props.set_attribute("for", "message");
							props.styles(&[Style::Block, NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.set_text("Message"));
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[Style::MarginTop(10)]);
							
							props.child("1", Dynamic).run("textarea", |props| {
								props.set_attribute("name", "message");
								props.set_attribute("id", "message");
								props.set_attribute("rows", "4");
								props.styles(&[Style::Block, NoStyle::Noop("w-full"), NoStyle::Noop("rounded-md"), NoStyle::Noop("border-0"), Style::PaddingX(14), Style::PaddingY(8), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), NoStyle::NoopGroup("placeholder", Style::TextColor(Color::Fg(44))), Action::Hover(&[NoStyle::Noop("ring-2")]), Action::Hover(&[NoStyle::Noop("ring-inset")]), Action::Hover(&[NoStyle::Noop("ring-indigo-600")]), Screen::Small(&[NoStyle::Noop("text-sm")]), Screen::Small(&[NoStyle::Noop("leading-6")])]);
							});
						});
					});
					props.child("13", Dynamic).run("div", |props| {
						props.styles(&[Style::Flex, NoStyle::Noop("gap-x-4"), Screen::Small(&[NoStyle::Noop("col-span-2")])]);
						props.set_attribute("x-data", "{ on: false }");
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[Style::Flex, Style::Width(24), Style::ItemsCenter]);
							
							props.child("1", Dynamic).run("button", |props| {
								props.set_attribute("type", "button");
								props.styles(&[Style::Color(Color::Fg(22)), Style::Flex, Style::Width(32), NoStyle::Noop("flex-none"), NoStyle::Noop("cursor-pointer"), NoStyle::Noop("rounded-full"), NoStyle::Noop("p-px"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-900/5"), NoStyle::Noop("transition-colors"), NoStyle::Noop("duration-200"), NoStyle::Noop("ease-in-out"), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-2")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-offset-2")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-indigo-600"))]);
								props.set_attribute("role", "switch");
								props.set_attribute("aria-checked", "false");
								props.set_attribute("x-ref", "switch");
								props.set_attribute("x-state:on", "Enabled");
								props.set_attribute("x-state:off", "Not Enabled");
								props.set_attribute(":class", "{ 'bg-indigo-600': on, 'bg-gray-200': !(on) }");
								props.set_attribute("aria-labelledby", "switch-1-label");
								props.set_attribute(":aria-checked", "on.toString()");
								props.set_attribute("@click", "on = !on");
								
								props.child("1", Dynamic).run("span", |props| {
									props.styles(&[NoStyle::Noop("sr-only")]);
									
									props.child("0", Label).run(|props| props.set_text("Agree to policies"));
								});
								props.child("3", Dynamic).run("span", |props| {
									props.set_attribute("aria-hidden", "true");
									props.styles(&[NoStyle::Noop("translate-x-0"), Style::Width(16), Style::Width(16), NoStyle::Noop("transform"), NoStyle::Noop("rounded-full"), NoStyle::Noop("bg-white"), NoStyle::Noop("shadow-sm"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-gray-900/5"), NoStyle::Noop("transition"), NoStyle::Noop("duration-200"), NoStyle::Noop("ease-in-out")]);
									props.set_attribute("x-state:on", "Enabled");
									props.set_attribute("x-state:off", "Not Enabled");
									props.set_attribute(":class", "{ 'translate-x-3.5': on, 'translate-x-0': !(on) }");
								});
							});
						});
						props.child("3", Dynamic).run("label", |props| {
							props.styles(&[NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(67))]);
							props.set_attribute("id", "switch-1-label");
							props.set_attribute("@click", "on = !on; $refs.switch.focus()");
							
							props.child("0", Label).run(|props| props.set_text("By selecting this, you agree to our"));
							//  space 
							props.child("3", Dynamic).run("a", |props| {
								props.set_attribute("href", "#");
								props.styles(&[Style::FontSemibold, NoStyle::Noop("text-indigo-600")]);
								
								props.child("0", Label).run(|props| props.set_text("privacy policy"));
							});
							props.child("4", Label).run(|props| props.set_text("."));
						});
					});
				});
				props.child("3", Dynamic).run("div", |props| {
					props.styles(&[Style::MarginTop(40)]);
					
					props.child("1", Dynamic).run("button", |props| {
						props.set_attribute("type", "submit");
						props.styles(&[Style::Block, NoStyle::Noop("w-full"), NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-indigo-600"), Style::PaddingX(14), Style::PaddingY(10), NoStyle::Noop("text-center"), NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("text-white"), NoStyle::Noop("shadow-sm"), Action::Hover(&[NoStyle::Noop("bg-indigo-500")]), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-2")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-offset-2")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-indigo-600"))]);
						
						props.child("0", Label).run(|props| props.set_text("Let's talk"));
					});
				});
			});
		});
	}
}
