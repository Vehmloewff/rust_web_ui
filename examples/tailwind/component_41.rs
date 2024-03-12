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
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[NoStyle::Noop("min-h-[640px]"), NoStyle::Noop("bg-white")]);
			
			props.child("1", Dynamic).run("header", |props| {
				props.set_attribute("x-data", "{ open: false }");
				props.set_attribute("@keydown.window.escape", "open = false");
				props.styles(&[NoStyle::Noop("bg-white")]);
				
				props.child("1", Dynamic).run("nav", |props| {
					props.styles(&[NoStyle::Noop("mx-auto"), Style::Flex, NoStyle::Noop("max-w-7xl"), Style::ItemsCenter, Style::JustifyBetween, Style::Padding(24), Screen::Large(&[Style::PaddingX(32)])]);
					props.set_attribute("aria-label", "Global");
					
					props.child("1", Dynamic).run("div", |props| {
						props.styles(&[Style::Flex, Screen::Large(&[NoStyle::Noop("flex-1")])]);
						
						props.child("1", Dynamic).run("a", |props| {
							props.set_attribute("href", "#");
							props.styles(&[NoStyle::Noop("-m-1.5"), Style::Padding(6)]);
							
							props.child("1", Dynamic).run("span", |props| {
								props.styles(&[NoStyle::Noop("sr-only")]);
								
								props.child("0", Label).run(|props| props.set_text("Your Company"));
							});
							props.child("3", Dynamic).run("img", |props| {
								props.styles(&[Style::Width(32), NoStyle::Noop("w-auto")]);
								props.set_attribute("src", "https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600");
								props.set_attribute("alt", "");
							});
						});
					});
					props.child("3", Dynamic).run("div", |props| {
						props.styles(&[Style::Flex, Screen::Large(&[NoStyle::Noop("hidden")])]);
						
						props.child("1", Dynamic).run("button", |props| {
							props.set_attribute("type", "button");
							props.styles(&[NoStyle::Noop("-m-2.5"), Style::InlineFlex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-md"), Style::Padding(10), Style::TextColor(Color::Fg(78))]);
							props.set_attribute("@click", "open = true");
							
							props.child("1", Dynamic).run("span", |props| {
								props.styles(&[NoStyle::Noop("sr-only")]);
								
								props.child("0", Label).run(|props| props.set_text("Open main menu"));
							});
							props.child("3", Icon).run(|props| {
								props.style(&[Style::Width(24), Style::Width(24)]);
							});
						});
					});
					props.child("5", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("hidden"), Screen::Large(&[Style::Flex]), Screen::Large(&[NoStyle::Noop("gap-x-12")])]);
						props.set_attribute("x-data", "Components.popoverGroup()");
						props.set_attribute("x-init", "init()");
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("relative")]);
							props.set_attribute("x-data", "Components.popover({ open: true, focus: false })");
							props.set_attribute("x-init", "init()");
							props.set_attribute("@keydown.escape", "onEscape");
							props.set_attribute("@close-popover-group.window", "onClosePopoverGroup");
							
							props.child("1", Dynamic).run("button", |props| {
								props.set_attribute("type", "button");
								props.styles(&[Style::Flex, Style::ItemsCenter, NoStyle::Noop("gap-x-1"), NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
								props.set_attribute("@click", "toggle");
								props.set_attribute("@mousedown", "if (open) $event.preventDefault()");
								props.set_attribute("aria-expanded", "false");
								props.set_attribute(":aria-expanded", "open.toString()");
								
								props.child("0", Label).run(|props| props.set_text("Product"));
								props.child("1", Icon).run(|props| {
									props.style(&[Style::Width(20), Style::Width(20), NoStyle::Noop("flex-none"), Style::TextColor(Color::Fg(44))]);
								});
							});
							props.child("3", Dynamic).run("div", |props| {
								props.set_attribute("x-show", "open");
								props.set_attribute("x-transition:enter", "transition ease-out duration-200");
								props.set_attribute("x-transition:enter-start", "opacity-0 translate-y-1");
								props.set_attribute("x-transition:enter-end", "opacity-100 translate-y-0");
								props.set_attribute("x-transition:leave", "transition ease-in duration-150");
								props.set_attribute("x-transition:leave-start", "opacity-100 translate-y-0");
								props.set_attribute("x-transition:leave-end", "opacity-0 translate-y-1");
								props.set_attribute("x-description", "'Product' flyout menu, show/hide based on flyout menu state.");
								props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("-left-8"), NoStyle::Noop("top-full"), NoStyle::Noop("z-10"), Style::MarginTop(12), NoStyle::Noop("w-screen"), NoStyle::Noop("max-w-md"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-3xl"), NoStyle::Noop("bg-white"), NoStyle::Noop("shadow-lg"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-gray-900/5")]);
								props.set_attribute("x-ref", "panel");
								props.set_attribute("@click.away", "open = false");
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[Style::Padding(16)]);
									
									props.child("1", Dynamic).run("div", |props| {
										props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), Style::Flex, Style::ItemsCenter, NoStyle::Noop("gap-x-6"), NoStyle::Noop("rounded-lg"), Style::Padding(16), NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Action::Hover(&[Style::Color(Color::Fg(6))])]);
										
										props.child("1", Dynamic).run("div", |props| {
											props.styles(&[Style::Flex, Style::Width(44), Style::Width(44), NoStyle::Noop("flex-none"), Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(6)), NoStyle::NoopGroup("group-hover", NoStyle::Noop("bg-white"))]);
											
											props.child("1", Icon).run(|props| {
												props.style(&[Style::Width(24), Style::Width(24), Style::TextColor(Color::Fg(67)), NoStyle::NoopGroup("group-hover", NoStyle::Noop("text-indigo-600"))]);
											});
										});
										props.child("3", Dynamic).run("div", |props| {
											props.styles(&[NoStyle::Noop("flex-auto")]);
											
											props.child("1", Dynamic).run("a", |props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::Block, Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
												
												props.child("0", Label).run(|props| props.set_text("Analytics"));
												props.child("1", Dynamic).run("span", |props| {
													props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0")]);
												});
											});
											props.child("3", Dynamic).run("p", |props| {
												props.styles(&[Style::MarginTop(4), Style::TextColor(Color::Fg(67))]);
												
												props.child("0", Label).run(|props| props.set_text("Get a better understanding of your traffic"));
											});
										});
									});
									props.child("3", Dynamic).run("div", |props| {
										props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), Style::Flex, Style::ItemsCenter, NoStyle::Noop("gap-x-6"), NoStyle::Noop("rounded-lg"), Style::Padding(16), NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Action::Hover(&[Style::Color(Color::Fg(6))])]);
										
										props.child("1", Dynamic).run("div", |props| {
											props.styles(&[Style::Flex, Style::Width(44), Style::Width(44), NoStyle::Noop("flex-none"), Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(6)), NoStyle::NoopGroup("group-hover", NoStyle::Noop("bg-white"))]);
											
											props.child("1", Icon).run(|props| {
												props.style(&[Style::Width(24), Style::Width(24), Style::TextColor(Color::Fg(67)), NoStyle::NoopGroup("group-hover", NoStyle::Noop("text-indigo-600"))]);
											});
										});
										props.child("3", Dynamic).run("div", |props| {
											props.styles(&[NoStyle::Noop("flex-auto")]);
											
											props.child("1", Dynamic).run("a", |props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::Block, Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
												
												props.child("0", Label).run(|props| props.set_text("Engagement"));
												props.child("1", Dynamic).run("span", |props| {
													props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0")]);
												});
											});
											props.child("3", Dynamic).run("p", |props| {
												props.styles(&[Style::MarginTop(4), Style::TextColor(Color::Fg(67))]);
												
												props.child("0", Label).run(|props| props.set_text("Speak directly to your customers"));
											});
										});
									});
									props.child("5", Dynamic).run("div", |props| {
										props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), Style::Flex, Style::ItemsCenter, NoStyle::Noop("gap-x-6"), NoStyle::Noop("rounded-lg"), Style::Padding(16), NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Action::Hover(&[Style::Color(Color::Fg(6))])]);
										
										props.child("1", Dynamic).run("div", |props| {
											props.styles(&[Style::Flex, Style::Width(44), Style::Width(44), NoStyle::Noop("flex-none"), Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(6)), NoStyle::NoopGroup("group-hover", NoStyle::Noop("bg-white"))]);
											
											props.child("1", Icon).run(|props| {
												props.style(&[Style::Width(24), Style::Width(24), Style::TextColor(Color::Fg(67)), NoStyle::NoopGroup("group-hover", NoStyle::Noop("text-indigo-600"))]);
											});
										});
										props.child("3", Dynamic).run("div", |props| {
											props.styles(&[NoStyle::Noop("flex-auto")]);
											
											props.child("1", Dynamic).run("a", |props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::Block, Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
												
												props.child("0", Label).run(|props| props.set_text("Security"));
												props.child("1", Dynamic).run("span", |props| {
													props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0")]);
												});
											});
											props.child("3", Dynamic).run("p", |props| {
												props.styles(&[Style::MarginTop(4), Style::TextColor(Color::Fg(67))]);
												
												props.child("0", Label).run(|props| props.set_text("Your customers’ data will be safe and secure"));
											});
										});
									});
									props.child("7", Dynamic).run("div", |props| {
										props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), Style::Flex, Style::ItemsCenter, NoStyle::Noop("gap-x-6"), NoStyle::Noop("rounded-lg"), Style::Padding(16), NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Action::Hover(&[Style::Color(Color::Fg(6))])]);
										
										props.child("1", Dynamic).run("div", |props| {
											props.styles(&[Style::Flex, Style::Width(44), Style::Width(44), NoStyle::Noop("flex-none"), Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(6)), NoStyle::NoopGroup("group-hover", NoStyle::Noop("bg-white"))]);
											
											props.child("1", Icon).run(|props| {
												props.style(&[Style::Width(24), Style::Width(24), Style::TextColor(Color::Fg(67)), NoStyle::NoopGroup("group-hover", NoStyle::Noop("text-indigo-600"))]);
											});
										});
										props.child("3", Dynamic).run("div", |props| {
											props.styles(&[NoStyle::Noop("flex-auto")]);
											
											props.child("1", Dynamic).run("a", |props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::Block, Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
												
												props.child("0", Label).run(|props| props.set_text("Integrations"));
												props.child("1", Dynamic).run("span", |props| {
													props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0")]);
												});
											});
											props.child("3", Dynamic).run("p", |props| {
												props.styles(&[Style::MarginTop(4), Style::TextColor(Color::Fg(67))]);
												
												props.child("0", Label).run(|props| props.set_text("Connect with third-party tools"));
											});
										});
									});
									props.child("9", Dynamic).run("div", |props| {
										props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), Style::Flex, Style::ItemsCenter, NoStyle::Noop("gap-x-6"), NoStyle::Noop("rounded-lg"), Style::Padding(16), NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), Action::Hover(&[Style::Color(Color::Fg(6))])]);
										
										props.child("1", Dynamic).run("div", |props| {
											props.styles(&[Style::Flex, Style::Width(44), Style::Width(44), NoStyle::Noop("flex-none"), Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(6)), NoStyle::NoopGroup("group-hover", NoStyle::Noop("bg-white"))]);
											
											props.child("1", Icon).run(|props| {
												props.style(&[Style::Width(24), Style::Width(24), Style::TextColor(Color::Fg(67)), NoStyle::NoopGroup("group-hover", NoStyle::Noop("text-indigo-600"))]);
											});
										});
										props.child("3", Dynamic).run("div", |props| {
											props.styles(&[NoStyle::Noop("flex-auto")]);
											
											props.child("1", Dynamic).run("a", |props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::Block, Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
												
												props.child("0", Label).run(|props| props.set_text("Automations"));
												props.child("1", Dynamic).run("span", |props| {
													props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0")]);
												});
											});
											props.child("3", Dynamic).run("p", |props| {
												props.styles(&[Style::MarginTop(4), Style::TextColor(Color::Fg(67))]);
												
												props.child("0", Label).run(|props| props.set_text("Build strategic funnels that will convert"));
											});
										});
									});
								});
								props.child("3", Dynamic).run("div", |props| {
									props.styles(&[NoStyle::Noop("grid"), NoStyle::Noop("grid-cols-2"), NoStyle::Noop("divide-x"), NoStyle::Noop("divide-gray-900/5"), Style::Color(Color::Fg(6))]);
									
									props.child("1", Dynamic).run("a", |props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("gap-x-2.5"), Style::Padding(12), NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100)), Action::Hover(&[Style::Color(Color::Fg(11))])]);
										
										props.child("1", Icon).run(|props| {
											props.style(&[Style::Width(20), Style::Width(20), NoStyle::Noop("flex-none"), Style::TextColor(Color::Fg(44))]);
										});
										props.child("2", Label).run(|props| props.set_text("Watch demo"));
									});
									props.child("3", Dynamic).run("a", |props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("gap-x-2.5"), Style::Padding(12), NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100)), Action::Hover(&[Style::Color(Color::Fg(11))])]);
										
										props.child("1", Icon).run(|props| {
											props.style(&[Style::Width(20), Style::Width(20), NoStyle::Noop("flex-none"), Style::TextColor(Color::Fg(44))]);
										});
										props.child("2", Label).run(|props| props.set_text("Contact sales"));
									});
								});
							});
						});
						props.child("3", Dynamic).run("a", |props| {
							props.set_attribute("href", "#");
							props.styles(&[NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.set_text("Features"));
						});
						props.child("5", Dynamic).run("a", |props| {
							props.set_attribute("href", "#");
							props.styles(&[NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.set_text("Marketplace"));
						});
						props.child("7", Dynamic).run("a", |props| {
							props.set_attribute("href", "#");
							props.styles(&[NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.set_text("Company"));
						});
					});
					props.child("7", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("hidden"), Screen::Large(&[Style::Flex]), Screen::Large(&[NoStyle::Noop("flex-1")]), Screen::Large(&[NoStyle::Noop("justify-end")])]);
						
						props.child("1", Dynamic).run("a", |props| {
							props.set_attribute("href", "#");
							props.styles(&[NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
							
							props.child("0", Label).run(|props| props.set_text("Log in"));
							props.child("1", Dynamic).run("span", |props| {
								props.set_attribute("aria-hidden", "true");
								
								props.child("0", Label).run(|props| props.set_text("→"));
							});
						});
					});
				});
				props.child("3", Dynamic).run("div", |props| {
					props.set_attribute("x-description", "Mobile menu, show/hide based on menu open state.");
					props.styles(&[Screen::Large(&[NoStyle::Noop("hidden")])]);
					props.set_attribute("x-ref", "dialog");
					props.set_attribute("x-show", "open");
					props.set_attribute("aria-modal", "true");
					
					props.child("1", Dynamic).run("div", |props| {
						props.set_attribute("x-description", "Background backdrop, show/hide based on slide-over state.");
						props.styles(&[NoStyle::Noop("fixed"), NoStyle::Noop("inset-0"), NoStyle::Noop("z-10")]);
					});
					props.child("3", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("fixed"), NoStyle::Noop("inset-y-0"), NoStyle::Noop("right-0"), NoStyle::Noop("z-10"), NoStyle::Noop("w-full"), NoStyle::Noop("overflow-y-auto"), NoStyle::Noop("bg-white"), Style::PaddingX(24), Style::PaddingY(24), Screen::Small(&[NoStyle::Noop("max-w-sm")]), Screen::Small(&[NoStyle::Noop("ring-1")]), Screen::Small(&[NoStyle::Noop("ring-gray-900/10")])]);
						props.set_attribute("@click.away", "open = false");
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyBetween]);
							
							props.child("1", Dynamic).run("a", |props| {
								props.set_attribute("href", "#");
								props.styles(&[NoStyle::Noop("-m-1.5"), Style::Padding(6)]);
								
								props.child("1", Dynamic).run("span", |props| {
									props.styles(&[NoStyle::Noop("sr-only")]);
									
									props.child("0", Label).run(|props| props.set_text("Your Company"));
								});
								props.child("3", Dynamic).run("img", |props| {
									props.styles(&[Style::Width(32), NoStyle::Noop("w-auto")]);
									props.set_attribute("src", "https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600");
									props.set_attribute("alt", "");
								});
							});
							props.child("3", Dynamic).run("button", |props| {
								props.set_attribute("type", "button");
								props.styles(&[NoStyle::Noop("-m-2.5"), NoStyle::Noop("rounded-md"), Style::Padding(10), Style::TextColor(Color::Fg(78))]);
								props.set_attribute("@click", "open = false");
								
								props.child("1", Dynamic).run("span", |props| {
									props.styles(&[NoStyle::Noop("sr-only")]);
									
									props.child("0", Label).run(|props| props.set_text("Close menu"));
								});
								props.child("3", Icon).run(|props| {
									props.style(&[Style::Width(24), Style::Width(24)]);
								});
							});
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[Style::MarginTop(24), NoStyle::Noop("flow-root")]);
							
							props.child("1", Dynamic).run("div", |props| {
								props.styles(&[NoStyle::Noop("-my-6"), NoStyle::Noop("divide-y"), NoStyle::Noop("divide-gray-500/10")]);
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[Style::SpaceY(8), Style::PaddingY(24)]);
									
									props.child("1", Dynamic).run("div", |props| {
										props.set_attribute("x-data", "{ open: false }");
										props.styles(&[NoStyle::Noop("-mx-3")]);
										
										props.child("1", Dynamic).run("button", |props| {
											props.set_attribute("type", "button");
											props.styles(&[Style::Flex, NoStyle::Noop("w-full"), Style::ItemsCenter, Style::JustifyBetween, NoStyle::Noop("rounded-lg"), Style::PaddingY(8), Style::PaddingLeft(12), Style::PaddingRight(14), NoStyle::Noop("text-base"), Style::FontSemibold, NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(100)), Action::Hover(&[Style::Color(Color::Fg(6))])]);
											props.set_attribute("aria-controls", "disclosure-1");
											props.set_attribute("@click", "open = !open");
											props.set_attribute("aria-expanded", "false");
											props.set_attribute("x-bind:aria-expanded", "open.toString()");
											
											props.child("0", Label).run(|props| props.set_text("Product"));
											props.child("1", Icon).run(|props| {
												props.style(&[Style::Width(20), Style::Width(20), NoStyle::Noop("flex-none")]);
											});
										});
										props.child("3", Dynamic).run("div", |props| {
											props.styles(&[Style::MarginTop(8), Style::SpaceY(8)]);
											props.set_attribute("x-description", "'Product' sub-menu, show/hide based on menu state.");
											props.set_attribute("id", "disclosure-1");
											props.set_attribute("x-show", "open");
											
											props.child("1", Dynamic).run("a", |props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::Block, NoStyle::Noop("rounded-lg"), Style::PaddingY(8), Style::PaddingLeft(24), Style::PaddingRight(12), NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(100)), Action::Hover(&[Style::Color(Color::Fg(6))])]);
												
												props.child("0", Label).run(|props| props.set_text("Analytics"));
											});
											props.child("3", Dynamic).run("a", |props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::Block, NoStyle::Noop("rounded-lg"), Style::PaddingY(8), Style::PaddingLeft(24), Style::PaddingRight(12), NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(100)), Action::Hover(&[Style::Color(Color::Fg(6))])]);
												
												props.child("0", Label).run(|props| props.set_text("Engagement"));
											});
											props.child("5", Dynamic).run("a", |props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::Block, NoStyle::Noop("rounded-lg"), Style::PaddingY(8), Style::PaddingLeft(24), Style::PaddingRight(12), NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(100)), Action::Hover(&[Style::Color(Color::Fg(6))])]);
												
												props.child("0", Label).run(|props| props.set_text("Security"));
											});
											props.child("7", Dynamic).run("a", |props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::Block, NoStyle::Noop("rounded-lg"), Style::PaddingY(8), Style::PaddingLeft(24), Style::PaddingRight(12), NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(100)), Action::Hover(&[Style::Color(Color::Fg(6))])]);
												
												props.child("0", Label).run(|props| props.set_text("Integrations"));
											});
											props.child("9", Dynamic).run("a", |props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::Block, NoStyle::Noop("rounded-lg"), Style::PaddingY(8), Style::PaddingLeft(24), Style::PaddingRight(12), NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(100)), Action::Hover(&[Style::Color(Color::Fg(6))])]);
												
												props.child("0", Label).run(|props| props.set_text("Automations"));
											});
											props.child("11", Dynamic).run("a", |props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::Block, NoStyle::Noop("rounded-lg"), Style::PaddingY(8), Style::PaddingLeft(24), Style::PaddingRight(12), NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(100)), Action::Hover(&[Style::Color(Color::Fg(6))])]);
												
												props.child("0", Label).run(|props| props.set_text("Watch demo"));
											});
											props.child("13", Dynamic).run("a", |props| {
												props.set_attribute("href", "#");
												props.styles(&[Style::Block, NoStyle::Noop("rounded-lg"), Style::PaddingY(8), Style::PaddingLeft(24), Style::PaddingRight(12), NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(100)), Action::Hover(&[Style::Color(Color::Fg(6))])]);
												
												props.child("0", Label).run(|props| props.set_text("Contact sales"));
											});
										});
									});
									props.child("3", Dynamic).run("a", |props| {
										props.set_attribute("href", "#");
										props.styles(&[NoStyle::Noop("-mx-3"), Style::Block, NoStyle::Noop("rounded-lg"), Style::PaddingX(12), Style::PaddingY(8), NoStyle::Noop("text-base"), Style::FontSemibold, NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(100)), Action::Hover(&[Style::Color(Color::Fg(6))])]);
										
										props.child("0", Label).run(|props| props.set_text("Features"));
									});
									props.child("5", Dynamic).run("a", |props| {
										props.set_attribute("href", "#");
										props.styles(&[NoStyle::Noop("-mx-3"), Style::Block, NoStyle::Noop("rounded-lg"), Style::PaddingX(12), Style::PaddingY(8), NoStyle::Noop("text-base"), Style::FontSemibold, NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(100)), Action::Hover(&[Style::Color(Color::Fg(6))])]);
										
										props.child("0", Label).run(|props| props.set_text("Marketplace"));
									});
									props.child("7", Dynamic).run("a", |props| {
										props.set_attribute("href", "#");
										props.styles(&[NoStyle::Noop("-mx-3"), Style::Block, NoStyle::Noop("rounded-lg"), Style::PaddingX(12), Style::PaddingY(8), NoStyle::Noop("text-base"), Style::FontSemibold, NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(100)), Action::Hover(&[Style::Color(Color::Fg(6))])]);
										
										props.child("0", Label).run(|props| props.set_text("Company"));
									});
								});
								props.child("3", Dynamic).run("div", |props| {
									props.styles(&[Style::PaddingY(24)]);
									
									props.child("1", Dynamic).run("a", |props| {
										props.set_attribute("href", "#");
										props.styles(&[NoStyle::Noop("-mx-3"), Style::Block, NoStyle::Noop("rounded-lg"), Style::PaddingX(12), Style::PaddingY(10), NoStyle::Noop("text-base"), Style::FontSemibold, NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(100)), Action::Hover(&[Style::Color(Color::Fg(6))])]);
										
										props.child("0", Label).run(|props| props.set_text("Log in"));
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
