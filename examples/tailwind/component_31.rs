use rust_web_ui::*;

pub struct Example31;

pub struct Example31Props {}

impl Default for Example31Props {
	fn default() -> Example31Props {
		Example31Props { }
	}
}

impl Widget<'_> for Example31 {
	type Props = Example31Props;

	fn render(mut ctx: Ctx<'_>, props: Example31Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[NoStyle::Noop("bg-white")]);
			props.set_attribute("style", "min-height: 712px");
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::Padding(32)]);
				
				props.child("1", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("relative")]);
					props.set_attribute("x-data", "Components.popover({ open: true, focus: false })");
					props.set_attribute("x-init", "init()");
					props.set_attribute("@keydown.escape", "onEscape");
					props.set_attribute("@close-popover-group.window", "onClosePopoverGroup");
					
					props.child("1", Dynamic).run("button", |props| {
						props.set_attribute("type", "button");
						props.styles(&[Style::InlineFlex, Style::ItemsCenter, NoStyle::Noop("gap-x-1"), NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
						props.set_attribute("@click", "toggle");
						props.set_attribute("@mousedown", "if (open) $event.preventDefault()");
						props.set_attribute("aria-expanded", "false");
						props.set_attribute(":aria-expanded", "open.toString()");
						
						props.child("1", Dynamic).run("span", |props| {
							props.child("0", Label).run(|props| props.set_text("Solutions"));
						});
						props.child("3", Icon).run(|props| {
							props.style(&[Style::Width(20), Style::Width(20)]);
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
						props.set_attribute("x-description", "Flyout menu, show/hide based on flyout menu state.");
						props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("left-1/2"), NoStyle::Noop("z-10"), Style::MarginTop(20), Style::Flex, NoStyle::Noop("w-screen"), NoStyle::Noop("max-w-max"), NoStyle::Noop("-translate-x-1/2"), Style::PaddingX(16)]);
						props.set_attribute("x-ref", "panel");
						props.set_attribute("@click.away", "open = false");
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("w-screen"), NoStyle::Noop("max-w-md"), NoStyle::Noop("flex-auto"), NoStyle::Noop("overflow-hidden"), NoStyle::Noop("rounded-3xl"), NoStyle::Noop("bg-white"), NoStyle::Noop("text-sm"), NoStyle::Noop("leading-6"), NoStyle::Noop("shadow-lg"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-gray-900/5")]);
							
							props.child("1", Dynamic).run("div", |props| {
								props.styles(&[Style::Padding(16)]);
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), Style::Flex, NoStyle::Noop("gap-x-6"), NoStyle::Noop("rounded-lg"), Style::Padding(16), Action::Hover(&[Style::Color(Color::Fg(6))])]);
									
									props.child("1", Dynamic).run("div", |props| {
										props.styles(&[Style::MarginTop(4), Style::Flex, Style::Width(44), Style::Width(44), NoStyle::Noop("flex-none"), Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(6)), NoStyle::NoopGroup("group-hover", NoStyle::Noop("bg-white"))]);
										
										props.child("1", Icon).run(|props| {
											props.style(&[Style::Width(24), Style::Width(24), Style::TextColor(Color::Fg(67)), NoStyle::NoopGroup("group-hover", NoStyle::Noop("text-indigo-600"))]);
										});
									});
									props.child("3", Dynamic).run("div", |props| {
										props.child("1", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
											
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
									props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), Style::Flex, NoStyle::Noop("gap-x-6"), NoStyle::Noop("rounded-lg"), Style::Padding(16), Action::Hover(&[Style::Color(Color::Fg(6))])]);
									
									props.child("1", Dynamic).run("div", |props| {
										props.styles(&[Style::MarginTop(4), Style::Flex, Style::Width(44), Style::Width(44), NoStyle::Noop("flex-none"), Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(6)), NoStyle::NoopGroup("group-hover", NoStyle::Noop("bg-white"))]);
										
										props.child("1", Icon).run(|props| {
											props.style(&[Style::Width(24), Style::Width(24), Style::TextColor(Color::Fg(67)), NoStyle::NoopGroup("group-hover", NoStyle::Noop("text-indigo-600"))]);
										});
									});
									props.child("3", Dynamic).run("div", |props| {
										props.child("1", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
											
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
									props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), Style::Flex, NoStyle::Noop("gap-x-6"), NoStyle::Noop("rounded-lg"), Style::Padding(16), Action::Hover(&[Style::Color(Color::Fg(6))])]);
									
									props.child("1", Dynamic).run("div", |props| {
										props.styles(&[Style::MarginTop(4), Style::Flex, Style::Width(44), Style::Width(44), NoStyle::Noop("flex-none"), Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(6)), NoStyle::NoopGroup("group-hover", NoStyle::Noop("bg-white"))]);
										
										props.child("1", Icon).run(|props| {
											props.style(&[Style::Width(24), Style::Width(24), Style::TextColor(Color::Fg(67)), NoStyle::NoopGroup("group-hover", NoStyle::Noop("text-indigo-600"))]);
										});
									});
									props.child("3", Dynamic).run("div", |props| {
										props.child("1", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
											
											props.child("0", Label).run(|props| props.set_text("Security"));
											props.child("1", Dynamic).run("span", |props| {
												props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-0")]);
											});
										});
										props.child("3", Dynamic).run("p", |props| {
											props.styles(&[Style::MarginTop(4), Style::TextColor(Color::Fg(67))]);
											
											props.child("0", Label).run(|props| props.set_text("Your customers' data will be safe and secure"));
										});
									});
								});
								props.child("7", Dynamic).run("div", |props| {
									props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), Style::Flex, NoStyle::Noop("gap-x-6"), NoStyle::Noop("rounded-lg"), Style::Padding(16), Action::Hover(&[Style::Color(Color::Fg(6))])]);
									
									props.child("1", Dynamic).run("div", |props| {
										props.styles(&[Style::MarginTop(4), Style::Flex, Style::Width(44), Style::Width(44), NoStyle::Noop("flex-none"), Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(6)), NoStyle::NoopGroup("group-hover", NoStyle::Noop("bg-white"))]);
										
										props.child("1", Icon).run(|props| {
											props.style(&[Style::Width(24), Style::Width(24), Style::TextColor(Color::Fg(67)), NoStyle::NoopGroup("group-hover", NoStyle::Noop("text-indigo-600"))]);
										});
									});
									props.child("3", Dynamic).run("div", |props| {
										props.child("1", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
											
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
									props.styles(&[NoStyle::Noop("group"), NoStyle::Noop("relative"), Style::Flex, NoStyle::Noop("gap-x-6"), NoStyle::Noop("rounded-lg"), Style::Padding(16), Action::Hover(&[Style::Color(Color::Fg(6))])]);
									
									props.child("1", Dynamic).run("div", |props| {
										props.styles(&[Style::MarginTop(4), Style::Flex, Style::Width(44), Style::Width(44), NoStyle::Noop("flex-none"), Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-lg"), Style::Color(Color::Fg(6)), NoStyle::NoopGroup("group-hover", NoStyle::Noop("bg-white"))]);
										
										props.child("1", Icon).run(|props| {
											props.style(&[Style::Width(24), Style::Width(24), Style::TextColor(Color::Fg(67)), NoStyle::NoopGroup("group-hover", NoStyle::Noop("text-indigo-600"))]);
										});
									});
									props.child("3", Dynamic).run("div", |props| {
										props.child("1", Dynamic).run("a", |props| {
											props.set_attribute("href", "#");
											props.styles(&[Style::FontSemibold, Style::TextColor(Color::Fg(100))]);
											
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
									props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("gap-x-2.5"), Style::Padding(12), Style::FontSemibold, Style::TextColor(Color::Fg(100)), Action::Hover(&[Style::Color(Color::Fg(11))])]);
									
									props.child("1", Icon).run(|props| {
										props.style(&[Style::Width(20), Style::Width(20), NoStyle::Noop("flex-none"), Style::TextColor(Color::Fg(44))]);
									});
									props.child("2", Label).run(|props| props.set_text("Watch demo"));
								});
								props.child("3", Dynamic).run("a", |props| {
									props.set_attribute("href", "#");
									props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("gap-x-2.5"), Style::Padding(12), Style::FontSemibold, Style::TextColor(Color::Fg(100)), Action::Hover(&[Style::Color(Color::Fg(11))])]);
									
									props.child("1", Icon).run(|props| {
										props.style(&[Style::Width(20), Style::Width(20), NoStyle::Noop("flex-none"), Style::TextColor(Color::Fg(44))]);
									});
									props.child("2", Label).run(|props| props.set_text("Contact sales"));
								});
							});
						});
					});
				});
			});
		});
	}
}
