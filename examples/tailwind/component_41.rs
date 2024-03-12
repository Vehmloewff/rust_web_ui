use rust_web_ui::*;

pub struct Example41;

pub struct Example41Props {}

impl Default for Example41Props {
	fn default() -> Example41Props {
		Example41Props { }
	}
}

impl Widget<'_> for Example41 {
	type Props = Example41Props;

	fn render(mut ctx: Ctx<'_>, props: Example41Props) {
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Noop("min-h-[640px]"), Style::Noop("bg-white")]);
			
			props.child("1", Dynamic).run("header", |mut props| {
				props.set_attribute("x-data", "{ open: false }");
				props.set_attribute("@keydown.window.escape", "open = false");
				props.styles(&[Style::Noop("bg-white")]);
				
				props.child("1", Dynamic).run("nav", |mut props| {
					props.styles(&[Style::Noop("mx-auto"), Style::Flex, Style::Noop("max-w-7xl"), Style::ItemsCenter, Style::JustifyBetween, Style::Padding(Size::Exact(24)), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
					props.set_attribute("aria-label", "Global");
					
					props.child("1", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Flex, Style::OnScreen(Screen::Large, &[Style::Noop("flex-1")])]);
						
						props.child("1", Dynamic).run("a", |mut props| {
							props.set_attribute("href", "#");
							props.styles(&[Style::Noop("-m-1.5"), Style::Padding(Size::Exact(6))]);
							
							props.child("1", Dynamic).run("span", |mut props| {
								props.styles(&[Style::Noop("sr-only")]);
								
								props.child("0", Label).run(|props| props.text("Your Company"));
							});
							props.child("3", Dynamic).run("img", |mut props| {
								props.styles(&[Style::Width(Size::Exact(32)), Style::Noop("w-auto")]);
								props.set_attribute("src", "https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600");
								props.set_attribute("alt", "");
							});
						});
					});
					props.child("3", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Flex, Style::OnScreen(Screen::Large, &[Style::Noop("hidden")])]);
						
						props.child("1", Dynamic).run("button", |mut props| {
							props.set_attribute("type", "button");
							props.styles(&[Style::Noop("-m-2.5"), Style::InlineFlex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-md"), Style::Padding(Size::Exact(10)), Style::TextColor(Color::Fg(78))]);
							props.set_attribute("@click", "open = true");
							
							props.child("1", Dynamic).run("span", |mut props| {
								props.styles(&[Style::Noop("sr-only")]);
								
								props.child("0", Label).run(|props| props.text("Open main menu"));
							});
							props.child("3", Icon).run(|mut props| {
								props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24))]);
							});
						});
					});
					props.child("5", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("hidden"), Style::OnScreen(Screen::Large, &[Style::Flex]), Style::OnScreen(Screen::Large, &[Style::Noop("gap-x-12")])]);
						props.set_attribute("x-data", "Components.popoverGroup()");
						props.set_attribute("x-init", "init()");
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("relative")]);
							props.set_attribute("x-data", "Components.popover({ open: true, focus: false })");
							props.set_attribute("x-init", "init()");
							props.set_attribute("@keydown.escape", "onEscape");
							props.set_attribute("@close-popover-group.window", "onClosePopoverGroup");
							
							props.child("1", Dynamic).run("button", |mut props| {
								props.set_attribute("type", "button");
								props.styles(&[Style::Flex, Style::ItemsCenter, Style::Noop("gap-x-1"), Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
								props.set_attribute("@click", "toggle");
								props.set_attribute("@mousedown", "if (open) $event.preventDefault()");
								props.set_attribute("aria-expanded", "false");
								props.set_attribute(":aria-expanded", "open.toString()");
								
								props.child("0", Label).run(|props| props.text("Product"));
								props.child("1", Icon).run(|mut props| {
									props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-none"), Style::TextColor(Color::Fg(44))]);
								});
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.set_attribute("x-show", "open");
								props.set_attribute("x-transition:enter", "transition ease-out duration-200");
								props.set_attribute("x-transition:enter-start", "opacity-0 translate-y-1");
								props.set_attribute("x-transition:enter-end", "opacity-100 translate-y-0");
								props.set_attribute("x-transition:leave", "transition ease-in duration-150");
								props.set_attribute("x-transition:leave-start", "opacity-100 translate-y-0");
								props.set_attribute("x-transition:leave-end", "opacity-0 translate-y-1");
								props.set_attribute("x-description", "'Product' flyout menu, show/hide based on flyout menu state.");
								props.styles(&[Style::Noop("absolute"), Style::Noop("-left-8"), Style::Noop("top-full"), Style::Noop("z-10"), Style::MarginTop(Size::Exact(12)), Style::Noop("w-screen"), Style::Noop("max-w-md"), Style::Noop("overflow-hidden"), Style::Noop("rounded-3xl"), Style::Noop("bg-white"), Style::Noop("shadow-lg"), Style::Noop("ring-1"), Style::Noop("ring-gray-900/5")]);
								props.set_attribute("x-ref", "panel");
								props.set_attribute("@click.away", "open = false");
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Padding(Size::Exact(16))]);
									
									props.child("1", Dynamic).run("div", |mut props| {
										props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::Noop("gap-x-6"), Style::Noop("rounded-lg"), Style::Padding(Size::Exact(16)), Style::Noop("text-sm"), Style::Noop("leading-6"), Style::OnHover(&[Style::Color(Color::Fg(6))])]);
										
										props.child("1", Dynamic).run("div", |mut props| {
											props.styles(&[Style::Flex, Style::Width(Size::Exact(44)), Style::Width(Size::Exact(44)), Style::Noop("flex-none"), Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-lg"), Style::Color(Color::Fg(6)), Style::NoopGroup("group-hover", &[Style::Noop("bg-white")])]);
											
											props.child("1", Icon).run(|mut props| {
												props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24)), Style::TextColor(Color::Fg(67)), Style::NoopGroup("group-hover", &[Style::Noop("text-indigo-600")])]);
											});
										});
										props.child("3", Dynamic).run("div", |mut props| {
											props.styles(&[Style::Noop("flex-auto")]);
											
											props.child("1", Dynamic).run("a", |mut props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::Block, Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
												
												props.child("0", Label).run(|props| props.text("Analytics"));
												props.child("1", Dynamic).run("span", |mut props| {
													props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0")]);
												});
											});
											props.child("3", Dynamic).run("p", |mut props| {
												props.styles(&[Style::MarginTop(Size::Exact(4)), Style::TextColor(Color::Fg(67))]);
												
												props.child("0", Label).run(|props| props.text("Get a better understanding of your traffic"));
											});
										});
									});
									props.child("3", Dynamic).run("div", |mut props| {
										props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::Noop("gap-x-6"), Style::Noop("rounded-lg"), Style::Padding(Size::Exact(16)), Style::Noop("text-sm"), Style::Noop("leading-6"), Style::OnHover(&[Style::Color(Color::Fg(6))])]);
										
										props.child("1", Dynamic).run("div", |mut props| {
											props.styles(&[Style::Flex, Style::Width(Size::Exact(44)), Style::Width(Size::Exact(44)), Style::Noop("flex-none"), Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-lg"), Style::Color(Color::Fg(6)), Style::NoopGroup("group-hover", &[Style::Noop("bg-white")])]);
											
											props.child("1", Icon).run(|mut props| {
												props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24)), Style::TextColor(Color::Fg(67)), Style::NoopGroup("group-hover", &[Style::Noop("text-indigo-600")])]);
											});
										});
										props.child("3", Dynamic).run("div", |mut props| {
											props.styles(&[Style::Noop("flex-auto")]);
											
											props.child("1", Dynamic).run("a", |mut props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::Block, Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
												
												props.child("0", Label).run(|props| props.text("Engagement"));
												props.child("1", Dynamic).run("span", |mut props| {
													props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0")]);
												});
											});
											props.child("3", Dynamic).run("p", |mut props| {
												props.styles(&[Style::MarginTop(Size::Exact(4)), Style::TextColor(Color::Fg(67))]);
												
												props.child("0", Label).run(|props| props.text("Speak directly to your customers"));
											});
										});
									});
									props.child("5", Dynamic).run("div", |mut props| {
										props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::Noop("gap-x-6"), Style::Noop("rounded-lg"), Style::Padding(Size::Exact(16)), Style::Noop("text-sm"), Style::Noop("leading-6"), Style::OnHover(&[Style::Color(Color::Fg(6))])]);
										
										props.child("1", Dynamic).run("div", |mut props| {
											props.styles(&[Style::Flex, Style::Width(Size::Exact(44)), Style::Width(Size::Exact(44)), Style::Noop("flex-none"), Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-lg"), Style::Color(Color::Fg(6)), Style::NoopGroup("group-hover", &[Style::Noop("bg-white")])]);
											
											props.child("1", Icon).run(|mut props| {
												props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24)), Style::TextColor(Color::Fg(67)), Style::NoopGroup("group-hover", &[Style::Noop("text-indigo-600")])]);
											});
										});
										props.child("3", Dynamic).run("div", |mut props| {
											props.styles(&[Style::Noop("flex-auto")]);
											
											props.child("1", Dynamic).run("a", |mut props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::Block, Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
												
												props.child("0", Label).run(|props| props.text("Security"));
												props.child("1", Dynamic).run("span", |mut props| {
													props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0")]);
												});
											});
											props.child("3", Dynamic).run("p", |mut props| {
												props.styles(&[Style::MarginTop(Size::Exact(4)), Style::TextColor(Color::Fg(67))]);
												
												props.child("0", Label).run(|props| props.text("Your customers’ data will be safe and secure"));
											});
										});
									});
									props.child("7", Dynamic).run("div", |mut props| {
										props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::Noop("gap-x-6"), Style::Noop("rounded-lg"), Style::Padding(Size::Exact(16)), Style::Noop("text-sm"), Style::Noop("leading-6"), Style::OnHover(&[Style::Color(Color::Fg(6))])]);
										
										props.child("1", Dynamic).run("div", |mut props| {
											props.styles(&[Style::Flex, Style::Width(Size::Exact(44)), Style::Width(Size::Exact(44)), Style::Noop("flex-none"), Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-lg"), Style::Color(Color::Fg(6)), Style::NoopGroup("group-hover", &[Style::Noop("bg-white")])]);
											
											props.child("1", Icon).run(|mut props| {
												props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24)), Style::TextColor(Color::Fg(67)), Style::NoopGroup("group-hover", &[Style::Noop("text-indigo-600")])]);
											});
										});
										props.child("3", Dynamic).run("div", |mut props| {
											props.styles(&[Style::Noop("flex-auto")]);
											
											props.child("1", Dynamic).run("a", |mut props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::Block, Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
												
												props.child("0", Label).run(|props| props.text("Integrations"));
												props.child("1", Dynamic).run("span", |mut props| {
													props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0")]);
												});
											});
											props.child("3", Dynamic).run("p", |mut props| {
												props.styles(&[Style::MarginTop(Size::Exact(4)), Style::TextColor(Color::Fg(67))]);
												
												props.child("0", Label).run(|props| props.text("Connect with third-party tools"));
											});
										});
									});
									props.child("9", Dynamic).run("div", |mut props| {
										props.styles(&[Style::Noop("group"), Style::Noop("relative"), Style::Flex, Style::ItemsCenter, Style::Noop("gap-x-6"), Style::Noop("rounded-lg"), Style::Padding(Size::Exact(16)), Style::Noop("text-sm"), Style::Noop("leading-6"), Style::OnHover(&[Style::Color(Color::Fg(6))])]);
										
										props.child("1", Dynamic).run("div", |mut props| {
											props.styles(&[Style::Flex, Style::Width(Size::Exact(44)), Style::Width(Size::Exact(44)), Style::Noop("flex-none"), Style::ItemsCenter, Style::JustifyCenter, Style::Noop("rounded-lg"), Style::Color(Color::Fg(6)), Style::NoopGroup("group-hover", &[Style::Noop("bg-white")])]);
											
											props.child("1", Icon).run(|mut props| {
												props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24)), Style::TextColor(Color::Fg(67)), Style::NoopGroup("group-hover", &[Style::Noop("text-indigo-600")])]);
											});
										});
										props.child("3", Dynamic).run("div", |mut props| {
											props.styles(&[Style::Noop("flex-auto")]);
											
											props.child("1", Dynamic).run("a", |mut props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::Block, Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
												
												props.child("0", Label).run(|props| props.text("Automations"));
												props.child("1", Dynamic).run("span", |mut props| {
													props.styles(&[Style::Noop("absolute"), Style::Noop("inset-0")]);
												});
											});
											props.child("3", Dynamic).run("p", |mut props| {
												props.styles(&[Style::MarginTop(Size::Exact(4)), Style::TextColor(Color::Fg(67))]);
												
												props.child("0", Label).run(|props| props.text("Build strategic funnels that will convert"));
											});
										});
									});
								});
								props.child("3", Dynamic).run("div", |mut props| {
									props.styles(&[Style::Noop("grid"), Style::Noop("grid-cols-2"), Style::Noop("divide-x"), Style::Noop("divide-gray-900/5"), Style::Color(Color::Fg(6))]);
									
									props.child("1", Dynamic).run("a", |mut props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("gap-x-2.5"), Style::Padding(Size::Exact(12)), Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::TextColor(Color::Fg(100)), Style::OnHover(&[Style::Color(Color::Fg(11))])]);
										
										props.child("1", Icon).run(|mut props| {
											props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-none"), Style::TextColor(Color::Fg(44))]);
										});
										props.child("2", Label).run(|props| props.text("Watch demo"));
									});
									props.child("3", Dynamic).run("a", |mut props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("gap-x-2.5"), Style::Padding(Size::Exact(12)), Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::TextColor(Color::Fg(100)), Style::OnHover(&[Style::Color(Color::Fg(11))])]);
										
										props.child("1", Icon).run(|mut props| {
											props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-none"), Style::TextColor(Color::Fg(44))]);
										});
										props.child("2", Label).run(|props| props.text("Contact sales"));
									});
								});
							});
						});
						props.child("3", Dynamic).run("a", |mut props| {
							props.set_attribute("href", "#");
							props.styles(&[Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.text("Features"));
						});
						props.child("5", Dynamic).run("a", |mut props| {
							props.set_attribute("href", "#");
							props.styles(&[Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.text("Marketplace"));
						});
						props.child("7", Dynamic).run("a", |mut props| {
							props.set_attribute("href", "#");
							props.styles(&[Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.text("Company"));
						});
					});
					props.child("7", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("hidden"), Style::OnScreen(Screen::Large, &[Style::Flex]), Style::OnScreen(Screen::Large, &[Style::Noop("flex-1")]), Style::OnScreen(Screen::Large, &[Style::Noop("justify-end")])]);
						
						props.child("1", Dynamic).run("a", |mut props| {
							props.set_attribute("href", "#");
							props.styles(&[Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.text("Log in"));
							props.child("1", Dynamic).run("span", |mut props| {
								props.set_attribute("aria-hidden", "true");
								
								props.child("0", Label).run(|props| props.text("→"));
							});
						});
					});
				});
				props.child("3", Dynamic).run("div", |mut props| {
					props.set_attribute("x-description", "Mobile menu, show/hide based on menu open state.");
					props.styles(&[Style::OnScreen(Screen::Large, &[Style::Noop("hidden")])]);
					props.set_attribute("x-ref", "dialog");
					props.set_attribute("x-show", "open");
					props.set_attribute("aria-modal", "true");
					
					props.child("1", Dynamic).run("div", |mut props| {
						props.set_attribute("x-description", "Background backdrop, show/hide based on slide-over state.");
						props.styles(&[Style::Noop("fixed"), Style::Noop("inset-0"), Style::Noop("z-10")]);
					});
					props.child("3", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("fixed"), Style::Noop("inset-y-0"), Style::Noop("right-0"), Style::Noop("z-10"), Style::Width(Size::Full), Style::Noop("overflow-y-auto"), Style::Noop("bg-white"), Style::PaddingX(Size::Exact(24)), Style::PaddingY(Size::Exact(24)), Style::OnScreen(Screen::Small, &[Style::Noop("max-w-sm")]), Style::OnScreen(Screen::Small, &[Style::Noop("ring-1")]), Style::OnScreen(Screen::Small, &[Style::Noop("ring-gray-900/10")])]);
						props.set_attribute("@click.away", "open = false");
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyBetween]);
							
							props.child("1", Dynamic).run("a", |mut props| {
								props.set_attribute("href", "#");
								props.styles(&[Style::Noop("-m-1.5"), Style::Padding(Size::Exact(6))]);
								
								props.child("1", Dynamic).run("span", |mut props| {
									props.styles(&[Style::Noop("sr-only")]);
									
									props.child("0", Label).run(|props| props.text("Your Company"));
								});
								props.child("3", Dynamic).run("img", |mut props| {
									props.styles(&[Style::Width(Size::Exact(32)), Style::Noop("w-auto")]);
									props.set_attribute("src", "https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600");
									props.set_attribute("alt", "");
								});
							});
							props.child("3", Dynamic).run("button", |mut props| {
								props.set_attribute("type", "button");
								props.styles(&[Style::Noop("-m-2.5"), Style::Noop("rounded-md"), Style::Padding(Size::Exact(10)), Style::TextColor(Color::Fg(78))]);
								props.set_attribute("@click", "open = false");
								
								props.child("1", Dynamic).run("span", |mut props| {
									props.styles(&[Style::Noop("sr-only")]);
									
									props.child("0", Label).run(|props| props.text("Close menu"));
								});
								props.child("3", Icon).run(|mut props| {
									props.style(&[Style::Width(Size::Exact(24)), Style::Width(Size::Exact(24))]);
								});
							});
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(24)), Style::Noop("flow-root")]);
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.styles(&[Style::Noop("-my-6"), Style::Noop("divide-y"), Style::Noop("divide-gray-500/10")]);
								
								props.child("1", Dynamic).run("div", |mut props| {
									props.styles(&[Style::SpaceY(Size::Exact(8)), Style::PaddingY(Size::Exact(24))]);
									
									props.child("1", Dynamic).run("div", |mut props| {
										props.set_attribute("x-data", "{ open: false }");
										props.styles(&[Style::Noop("-mx-3")]);
										
										props.child("1", Dynamic).run("button", |mut props| {
											props.set_attribute("type", "button");
											props.styles(&[Style::Flex, Style::Width(Size::Full), Style::ItemsCenter, Style::JustifyBetween, Style::Noop("rounded-lg"), Style::PaddingY(Size::Exact(8)), Style::PaddingLeft(Size::Exact(12)), Style::PaddingRight(Size::Exact(14)), Style::Noop("text-base"), Style::FontSemibold, Style::Noop("leading-7"), Style::TextColor(Color::Fg(100)), Style::OnHover(&[Style::Color(Color::Fg(6))])]);
											props.set_attribute("aria-controls", "disclosure-1");
											props.set_attribute("@click", "open = !open");
											props.set_attribute("aria-expanded", "false");
											props.set_attribute("x-bind:aria-expanded", "open.toString()");
											
											props.child("0", Label).run(|props| props.text("Product"));
											props.child("1", Icon).run(|mut props| {
												props.style(&[Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-none")]);
											});
										});
										props.child("3", Dynamic).run("div", |mut props| {
											props.styles(&[Style::MarginTop(Size::Exact(8)), Style::SpaceY(Size::Exact(8))]);
											props.set_attribute("x-description", "'Product' sub-menu, show/hide based on menu state.");
											props.set_attribute("id", "disclosure-1");
											props.set_attribute("x-show", "open");
											
											props.child("1", Dynamic).run("a", |mut props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::Block, Style::Noop("rounded-lg"), Style::PaddingY(Size::Exact(8)), Style::PaddingLeft(Size::Exact(24)), Style::PaddingRight(Size::Exact(12)), Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-7"), Style::TextColor(Color::Fg(100)), Style::OnHover(&[Style::Color(Color::Fg(6))])]);
												
												props.child("0", Label).run(|props| props.text("Analytics"));
											});
											props.child("3", Dynamic).run("a", |mut props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::Block, Style::Noop("rounded-lg"), Style::PaddingY(Size::Exact(8)), Style::PaddingLeft(Size::Exact(24)), Style::PaddingRight(Size::Exact(12)), Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-7"), Style::TextColor(Color::Fg(100)), Style::OnHover(&[Style::Color(Color::Fg(6))])]);
												
												props.child("0", Label).run(|props| props.text("Engagement"));
											});
											props.child("5", Dynamic).run("a", |mut props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::Block, Style::Noop("rounded-lg"), Style::PaddingY(Size::Exact(8)), Style::PaddingLeft(Size::Exact(24)), Style::PaddingRight(Size::Exact(12)), Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-7"), Style::TextColor(Color::Fg(100)), Style::OnHover(&[Style::Color(Color::Fg(6))])]);
												
												props.child("0", Label).run(|props| props.text("Security"));
											});
											props.child("7", Dynamic).run("a", |mut props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::Block, Style::Noop("rounded-lg"), Style::PaddingY(Size::Exact(8)), Style::PaddingLeft(Size::Exact(24)), Style::PaddingRight(Size::Exact(12)), Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-7"), Style::TextColor(Color::Fg(100)), Style::OnHover(&[Style::Color(Color::Fg(6))])]);
												
												props.child("0", Label).run(|props| props.text("Integrations"));
											});
											props.child("9", Dynamic).run("a", |mut props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::Block, Style::Noop("rounded-lg"), Style::PaddingY(Size::Exact(8)), Style::PaddingLeft(Size::Exact(24)), Style::PaddingRight(Size::Exact(12)), Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-7"), Style::TextColor(Color::Fg(100)), Style::OnHover(&[Style::Color(Color::Fg(6))])]);
												
												props.child("0", Label).run(|props| props.text("Automations"));
											});
											props.child("11", Dynamic).run("a", |mut props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::Block, Style::Noop("rounded-lg"), Style::PaddingY(Size::Exact(8)), Style::PaddingLeft(Size::Exact(24)), Style::PaddingRight(Size::Exact(12)), Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-7"), Style::TextColor(Color::Fg(100)), Style::OnHover(&[Style::Color(Color::Fg(6))])]);
												
												props.child("0", Label).run(|props| props.text("Watch demo"));
											});
											props.child("13", Dynamic).run("a", |mut props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::Block, Style::Noop("rounded-lg"), Style::PaddingY(Size::Exact(8)), Style::PaddingLeft(Size::Exact(24)), Style::PaddingRight(Size::Exact(12)), Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("leading-7"), Style::TextColor(Color::Fg(100)), Style::OnHover(&[Style::Color(Color::Fg(6))])]);
												
												props.child("0", Label).run(|props| props.text("Contact sales"));
											});
										});
									});
									props.child("3", Dynamic).run("a", |mut props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::Noop("-mx-3"), Style::Block, Style::Noop("rounded-lg"), Style::PaddingX(Size::Exact(12)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-base"), Style::FontSemibold, Style::Noop("leading-7"), Style::TextColor(Color::Fg(100)), Style::OnHover(&[Style::Color(Color::Fg(6))])]);
										
										props.child("0", Label).run(|props| props.text("Features"));
									});
									props.child("5", Dynamic).run("a", |mut props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::Noop("-mx-3"), Style::Block, Style::Noop("rounded-lg"), Style::PaddingX(Size::Exact(12)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-base"), Style::FontSemibold, Style::Noop("leading-7"), Style::TextColor(Color::Fg(100)), Style::OnHover(&[Style::Color(Color::Fg(6))])]);
										
										props.child("0", Label).run(|props| props.text("Marketplace"));
									});
									props.child("7", Dynamic).run("a", |mut props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::Noop("-mx-3"), Style::Block, Style::Noop("rounded-lg"), Style::PaddingX(Size::Exact(12)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-base"), Style::FontSemibold, Style::Noop("leading-7"), Style::TextColor(Color::Fg(100)), Style::OnHover(&[Style::Color(Color::Fg(6))])]);
										
										props.child("0", Label).run(|props| props.text("Company"));
									});
								});
								props.child("3", Dynamic).run("div", |mut props| {
									props.styles(&[Style::PaddingY(Size::Exact(24))]);
									
									props.child("1", Dynamic).run("a", |mut props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::Noop("-mx-3"), Style::Block, Style::Noop("rounded-lg"), Style::PaddingX(Size::Exact(12)), Style::PaddingY(Size::Exact(10)), Style::Noop("text-base"), Style::FontSemibold, Style::Noop("leading-7"), Style::TextColor(Color::Fg(100)), Style::OnHover(&[Style::Color(Color::Fg(6))])]);
										
										props.child("0", Label).run(|props| props.text("Log in"));
									});
								});
							});
						});
					});
				});
			});
		});
	}
}
