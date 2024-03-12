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
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Noop("isolate"), Style::Noop("bg-white"), Style::PaddingX(Size::Exact(24)), Style::PaddingY(Size::Exact(96)), Style::OnScreen(Screen::Small, &[Style::PaddingY(Size::Exact(128))]), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("absolute"), Style::Noop("inset-x-0"), Style::Noop("top-[-10rem]"), Style::Noop("-z-10"), Style::Noop("transform-gpu"), Style::Noop("overflow-hidden"), Style::Noop("blur-3xl"), Style::OnScreen(Screen::Small, &[Style::Noop("top-[-20rem]")])]);
				props.set_attribute("aria-hidden", "true");
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("relative"), Style::Noop("left-1/2"), Style::Noop("-z-10"), Style::Noop("aspect-[1155/678]"), Style::Noop("w-[36.125rem]"), Style::Noop("max-w-none"), Style::Noop("-translate-x-1/2"), Style::Noop("rotate-[30deg]"), Style::Noop("bg-gradient-to-tr"), Style::Noop("from-[#ff80b5]"), Style::Noop("to-[#9089fc]"), Style::Noop("opacity-30"), Style::OnScreen(Screen::Small, &[Style::Noop("left-[calc(50%-40rem)]")]), Style::OnScreen(Screen::Small, &[Style::Noop("w-[72.1875rem]")])]);
					props.set_attribute("style", "clip-path: polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)");
				});
			});
			props.child("3", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-2xl"), Style::Noop("text-center")]);
				
				props.child("1", Dynamic).run("h2", |mut props| {
					props.styles(&[Style::Noop("text-3xl"), Style::FontBold, Style::Noop("tracking-tight"), Style::TextColor(Color::Fg(100)), Style::OnScreen(Screen::Small, &[Style::Noop("text-4xl")])]);
					
					props.child("0", Label).run(|props| props.text("Contact sales"));
				});
				props.child("3", Dynamic).run("p", |mut props| {
					props.styles(&[Style::MarginTop(Size::Exact(8)), Style::Noop("text-lg"), Style::Noop("leading-8"), Style::TextColor(Color::Fg(67))]);
					
					props.child("0", Label).run(|props| props.text("Aute magna irure deserunt veniam aliqua magna enim voluptate."));
				});
			});
			props.child("5", Dynamic).run("form", |mut props| {
				props.set_attribute("action", "#");
				props.set_attribute("method", "POST");
				props.styles(&[Style::Noop("mx-auto"), Style::MarginTop(Size::Exact(64)), Style::Noop("max-w-xl"), Style::OnScreen(Screen::Small, &[Style::MarginTop(Size::Exact(80))])]);
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.styles(&[Style::Noop("grid"), Style::Noop("grid-cols-1"), Style::Noop("gap-x-8"), Style::Noop("gap-y-6"), Style::OnScreen(Screen::Small, &[Style::Noop("grid-cols-2")])]);
					
					props.child("1", Dynamic).run("div", |mut props| {
						props.child("1", Dynamic).run("label", |mut props| {
							props.set_attribute("for", "first-name");
							props.styles(&[Style::Block, Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.text("First name"));
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(10))]);
							
							props.child("1", Dynamic).run("input", |mut props| {
								props.set_attribute("type", "text");
								props.set_attribute("name", "first-name");
								props.set_attribute("id", "first-name");
								props.set_attribute("autocomplete", "given-name");
								props.styles(&[Style::Block, Style::Width(Size::Full), Style::Noop("rounded-md"), Style::Noop("border-0"), Style::PaddingX(Size::Exact(14)), Style::PaddingY(Size::Exact(8)), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::NoopGroup("placeholder", &[Style::TextColor(Color::Fg(44))]), Style::OnFocus(&[Style::Noop("ring-2")]), Style::OnFocus(&[Style::Noop("ring-inset")]), Style::OnFocus(&[Style::Noop("ring-indigo-600")]), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")]), Style::OnScreen(Screen::Small, &[Style::Noop("leading-6")])]);
							});
						});
					});
					props.child("3", Dynamic).run("div", |mut props| {
						props.child("1", Dynamic).run("label", |mut props| {
							props.set_attribute("for", "last-name");
							props.styles(&[Style::Block, Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.text("Last name"));
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(10))]);
							
							props.child("1", Dynamic).run("input", |mut props| {
								props.set_attribute("type", "text");
								props.set_attribute("name", "last-name");
								props.set_attribute("id", "last-name");
								props.set_attribute("autocomplete", "family-name");
								props.styles(&[Style::Block, Style::Width(Size::Full), Style::Noop("rounded-md"), Style::Noop("border-0"), Style::PaddingX(Size::Exact(14)), Style::PaddingY(Size::Exact(8)), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::NoopGroup("placeholder", &[Style::TextColor(Color::Fg(44))]), Style::OnFocus(&[Style::Noop("ring-2")]), Style::OnFocus(&[Style::Noop("ring-inset")]), Style::OnFocus(&[Style::Noop("ring-indigo-600")]), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")]), Style::OnScreen(Screen::Small, &[Style::Noop("leading-6")])]);
							});
						});
					});
					props.child("5", Dynamic).run("div", |mut props| {
						props.styles(&[Style::OnScreen(Screen::Small, &[Style::Noop("col-span-2")])]);
						
						props.child("1", Dynamic).run("label", |mut props| {
							props.set_attribute("for", "company");
							props.styles(&[Style::Block, Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.text("Company"));
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(10))]);
							
							props.child("1", Dynamic).run("input", |mut props| {
								props.set_attribute("type", "text");
								props.set_attribute("name", "company");
								props.set_attribute("id", "company");
								props.set_attribute("autocomplete", "organization");
								props.styles(&[Style::Block, Style::Width(Size::Full), Style::Noop("rounded-md"), Style::Noop("border-0"), Style::PaddingX(Size::Exact(14)), Style::PaddingY(Size::Exact(8)), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::NoopGroup("placeholder", &[Style::TextColor(Color::Fg(44))]), Style::OnFocus(&[Style::Noop("ring-2")]), Style::OnFocus(&[Style::Noop("ring-inset")]), Style::OnFocus(&[Style::Noop("ring-indigo-600")]), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")]), Style::OnScreen(Screen::Small, &[Style::Noop("leading-6")])]);
							});
						});
					});
					props.child("7", Dynamic).run("div", |mut props| {
						props.styles(&[Style::OnScreen(Screen::Small, &[Style::Noop("col-span-2")])]);
						
						props.child("1", Dynamic).run("label", |mut props| {
							props.set_attribute("for", "email");
							props.styles(&[Style::Block, Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.text("Email"));
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(10))]);
							
							props.child("1", Dynamic).run("input", |mut props| {
								props.set_attribute("type", "email");
								props.set_attribute("name", "email");
								props.set_attribute("id", "email");
								props.set_attribute("autocomplete", "email");
								props.styles(&[Style::Block, Style::Width(Size::Full), Style::Noop("rounded-md"), Style::Noop("border-0"), Style::PaddingX(Size::Exact(14)), Style::PaddingY(Size::Exact(8)), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::NoopGroup("placeholder", &[Style::TextColor(Color::Fg(44))]), Style::OnFocus(&[Style::Noop("ring-2")]), Style::OnFocus(&[Style::Noop("ring-inset")]), Style::OnFocus(&[Style::Noop("ring-indigo-600")]), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")]), Style::OnScreen(Screen::Small, &[Style::Noop("leading-6")])]);
							});
						});
					});
					props.child("9", Dynamic).run("div", |mut props| {
						props.styles(&[Style::OnScreen(Screen::Small, &[Style::Noop("col-span-2")])]);
						
						props.child("1", Dynamic).run("label", |mut props| {
							props.set_attribute("for", "phone-number");
							props.styles(&[Style::Block, Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.text("Phone number"));
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("relative"), Style::MarginTop(Size::Exact(10))]);
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Noop("absolute"), Style::Noop("inset-y-0"), Style::Noop("left-0"), Style::Flex, Style::ItemsCenter]);
								
								props.child("1", Dynamic).run("label", |mut props| {
									props.set_attribute("for", "country");
									props.styles(&[Style::Noop("sr-only")]);
									
									props.child("0", Label).run(|props| props.text("Country"));
								});
								props.child("3", Dynamic).run("select", |mut props| {
									props.set_attribute("id", "country");
									props.set_attribute("name", "country");
									props.styles(&[Style::Width(Size::Full), Style::Noop("rounded-md"), Style::Noop("border-0"), Style::Noop("bg-transparent"), Style::Noop("bg-none"), Style::PaddingY(Size::Exact(0)), Style::PaddingLeft(Size::Exact(16)), Style::PaddingRight(Size::Exact(36)), Style::TextColor(Color::Fg(44)), Style::OnFocus(&[Style::Noop("ring-2")]), Style::OnFocus(&[Style::Noop("ring-inset")]), Style::OnFocus(&[Style::Noop("ring-indigo-600")]), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")])]);
									
									props.child("1", Dynamic).run("option", |mut props| {
										props.child("0", Label).run(|props| props.text("US"));
									});
									props.child("3", Dynamic).run("option", |mut props| {
										props.child("0", Label).run(|props| props.text("CA"));
									});
									props.child("5", Dynamic).run("option", |mut props| {
										props.child("0", Label).run(|props| props.text("EU"));
									});
								});
								props.child("5", Icon).run(|mut props| {
									props.style(&[Style::Noop("pointer-events-none"), Style::Noop("absolute"), Style::Noop("right-3"), Style::Noop("top-0"), Style::Width(Size::Full), Style::Width(Size::Exact(20)), Style::TextColor(Color::Fg(44))]);
								});
							});
							props.child("3", Dynamic).run("input", |mut props| {
								props.set_attribute("type", "tel");
								props.set_attribute("name", "phone-number");
								props.set_attribute("id", "phone-number");
								props.set_attribute("autocomplete", "tel");
								props.styles(&[Style::Block, Style::Width(Size::Full), Style::Noop("rounded-md"), Style::Noop("border-0"), Style::PaddingX(Size::Exact(14)), Style::PaddingY(Size::Exact(8)), Style::PaddingLeft(Size::Exact(80)), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::NoopGroup("placeholder", &[Style::TextColor(Color::Fg(44))]), Style::OnFocus(&[Style::Noop("ring-2")]), Style::OnFocus(&[Style::Noop("ring-inset")]), Style::OnFocus(&[Style::Noop("ring-indigo-600")]), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")]), Style::OnScreen(Screen::Small, &[Style::Noop("leading-6")])]);
							});
						});
					});
					props.child("11", Dynamic).run("div", |mut props| {
						props.styles(&[Style::OnScreen(Screen::Small, &[Style::Noop("col-span-2")])]);
						
						props.child("1", Dynamic).run("label", |mut props| {
							props.set_attribute("for", "message");
							props.styles(&[Style::Block, Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.text("Message"));
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(10))]);
							
							props.child("1", Dynamic).run("textarea", |mut props| {
								props.set_attribute("name", "message");
								props.set_attribute("id", "message");
								props.set_attribute("rows", "4");
								props.styles(&[Style::Block, Style::Width(Size::Full), Style::Noop("rounded-md"), Style::Noop("border-0"), Style::PaddingX(Size::Exact(14)), Style::PaddingY(Size::Exact(8)), Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::NoopGroup("placeholder", &[Style::TextColor(Color::Fg(44))]), Style::OnFocus(&[Style::Noop("ring-2")]), Style::OnFocus(&[Style::Noop("ring-inset")]), Style::OnFocus(&[Style::Noop("ring-indigo-600")]), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")]), Style::OnScreen(Screen::Small, &[Style::Noop("leading-6")])]);
							});
						});
					});
					props.child("13", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Flex, Style::Noop("gap-x-4"), Style::OnScreen(Screen::Small, &[Style::Noop("col-span-2")])]);
						props.set_attribute("x-data", "{ on: false }");
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Flex, Style::Width(Size::Exact(24)), Style::ItemsCenter]);
							
							props.child("1", Dynamic).run("button", |mut props| {
								props.set_attribute("type", "button");
								props.styles(&[Style::Color(Color::Fg(22)), Style::Flex, Style::Width(Size::Exact(32)), Style::Noop("flex-none"), Style::Noop("cursor-pointer"), Style::Noop("rounded-full"), Style::Noop("p-px"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-900/5"), Style::Noop("transition-colors"), Style::Noop("duration-200"), Style::Noop("ease-in-out"), Style::NoopGroup("focus-visible", &[Style::Noop("outline")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-2")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-offset-2")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-indigo-600")])]);
								props.set_attribute("role", "switch");
								props.set_attribute("aria-checked", "false");
								props.set_attribute("x-ref", "switch");
								props.set_attribute("x-state:on", "Enabled");
								props.set_attribute("x-state:off", "Not Enabled");
								props.set_attribute(":class", "{ 'bg-indigo-600': on, 'bg-gray-200': !(on) }");
								props.set_attribute("aria-labelledby", "switch-1-label");
								props.set_attribute(":aria-checked", "on.toString()");
								props.set_attribute("@click", "on = !on");
								
								props.child("1", Dynamic).run("span", |mut props| {
									props.styles(&[Style::Noop("sr-only")]);
									
									props.child("0", Label).run(|props| props.text("Agree to policies"));
								});
								props.child("3", Dynamic).run("span", |mut props| {
									props.set_attribute("aria-hidden", "true");
									props.styles(&[Style::Noop("translate-x-0"), Style::Width(Size::Exact(16)), Style::Width(Size::Exact(16)), Style::Noop("transform"), Style::Noop("rounded-full"), Style::Noop("bg-white"), Style::Noop("shadow-sm"), Style::Noop("ring-1"), Style::Noop("ring-gray-900/5"), Style::Noop("transition"), Style::Noop("duration-200"), Style::Noop("ease-in-out")]);
									props.set_attribute("x-state:on", "Enabled");
									props.set_attribute("x-state:off", "Not Enabled");
									props.set_attribute(":class", "{ 'translate-x-3.5': on, 'translate-x-0': !(on) }");
								});
							});
						});
						props.child("3", Dynamic).run("label", |mut props| {
							props.styles(&[Style::Noop("text-sm"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(67))]);
							props.set_attribute("id", "switch-1-label");
							props.set_attribute("@click", "on = !on; $refs.switch.focus()");
							
							props.child("0", Label).run(|props| props.text("By selecting this, you agree to our"));
							//  space 
							props.child("3", Dynamic).run("a", |mut props| {
								props.set_attribute("href", "#");
								props.styles(&[Style::FontSemibold, Style::Noop("text-indigo-600")]);
								
								props.child("0", Label).run(|props| props.text("privacy policy"));
							});
							props.child("4", Label).run(|props| props.text("."));
						});
					});
				});
				props.child("3", Dynamic).run("div", |mut props| {
					props.styles(&[Style::MarginTop(Size::Exact(40))]);
					
					props.child("1", Dynamic).run("button", |mut props| {
						props.set_attribute("type", "submit");
						props.styles(&[Style::Block, Style::Width(Size::Full), Style::Noop("rounded-md"), Style::Noop("bg-indigo-600"), Style::PaddingX(Size::Exact(14)), Style::PaddingY(Size::Exact(10)), Style::Noop("text-center"), Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("text-white"), Style::Noop("shadow-sm"), Style::OnHover(&[Style::Noop("bg-indigo-500")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-2")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-offset-2")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-indigo-600")])]);
						
						props.child("0", Label).run(|props| props.text("Let's talk"));
					});
				});
			});
		});
	}
}
